#!/usr/bin/env python3
"""
ML training script for famfin transaction categorization.

Trains a scikit-learn Naive Bayes classifier on household transaction history
and exports to ONNX format for inference on Pi 3B.

Usage:
    python scripts/train.py --export models/model.onnx [--data data/transactions.csv]
"""

import argparse
import csv
import os
import re
import sys
import time
from pathlib import Path
from typing import List, Tuple, Dict

import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.naive_bayes import MultinomialNB
from sklearn.preprocessing import LabelEncoder
from sklearn.pipeline import Pipeline
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score, precision_score, recall_score, confusion_matrix
import skl2onnx
from skl2onnx.common.data_types import StringTensorType
import onnx


def normalize_label(label: str) -> str:
    """
    Normalize merchant label for feature extraction.

    Matches the normalization applied in Rust inference layer.
    - lowercase
    - trim whitespace
    - remove punctuation, keep only alphanumeric + spaces
    - collapse multiple spaces to single space

    Args:
        label: Raw merchant label (e.g., "CARREFOUR S.A.R.L.")

    Returns:
        Normalized label (e.g., "carrefour sarl")
    """
    # Lowercase
    normalized = label.lower()
    # Trim
    normalized = normalized.strip()
    # Remove punctuation, keep alphanumeric + spaces
    normalized = re.sub(r'[^a-z0-9\s]', '', normalized)
    # Collapse multiple spaces
    normalized = ' '.join(normalized.split())
    return normalized


def load_training_data(csv_path: Path) -> Tuple[List[str], List[str]]:
    """
    Load training data from CSV.

    Expected CSV columns: date, amount, label (merchant), category
    - date: ISO 8601 format (YYYY-MM-DD) — not used in this version
    - amount: Transaction amount — not used in this version (PII boundary)
    - label: Merchant name (raw, will be normalized)
    - category: Target category (e.g., "Food", "Transport", "Utilities")

    Args:
        csv_path: Path to training data CSV

    Returns:
        (merchant_labels, categories) — normalized labels and target categories

    Raises:
        FileNotFoundError: If CSV file doesn't exist
        ValueError: If required columns are missing
    """
    if not csv_path.exists():
        raise FileNotFoundError(f"Training data not found: {csv_path}")

    merchant_labels = []
    categories = []

    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        if reader.fieldnames is None or not all(col in reader.fieldnames for col in ['label', 'category']):
            raise ValueError("CSV must have 'label' and 'category' columns")

        for row in reader:
            raw_label = row.get('label', '').strip()
            category = row.get('category', '').strip()

            if raw_label and category:
                # Normalize the merchant label
                normalized = normalize_label(raw_label)
                if normalized:  # Only add non-empty normalized labels
                    merchant_labels.append(normalized)
                    categories.append(category)

    if not merchant_labels:
        raise ValueError("No valid training data found in CSV")

    return merchant_labels, categories


def train_model(merchant_labels: List[str], categories: List[str]) -> Tuple[Pipeline, LabelEncoder, Dict]:
    """
    Train a Naive Bayes classifier on merchant labels.

    Pipeline:
    1. TfidfVectorizer: Convert merchant labels → sparse feature matrix
    2. MultinomialNB: Train probabilistic classifier

    Args:
        merchant_labels: Normalized merchant labels
        categories: Target category labels

    Returns:
        (pipeline, label_encoder, metrics_dict) where:
        - pipeline: Fitted scikit-learn Pipeline (vectorizer + classifier)
        - label_encoder: Fitted LabelEncoder (category → integer)
        - metrics_dict: {overall_accuracy, per_category_metrics, category_distribution}
    """
    # Encode categories to integers
    label_encoder = LabelEncoder()
    encoded_targets = label_encoder.fit_transform(categories)

    # Train/test split (80/20)
    # For small datasets, don't stratify to avoid "too few members" error
    min_samples_per_class = np.bincount(encoded_targets).min()
    use_stratify = len(np.unique(encoded_targets)) > 1 and min_samples_per_class >= 5

    X_train, X_test, y_train, y_test = train_test_split(
        merchant_labels,
        encoded_targets,
        test_size=0.2,
        random_state=42,
        stratify=encoded_targets if use_stratify else None
    )

    # Create pipeline: vectorizer → classifier
    pipeline = Pipeline([
        ('tfidf', TfidfVectorizer(lowercase=False, max_features=1000)),  # Already lowercased
        ('clf', MultinomialNB(alpha=0.1))
    ])

    # Fit pipeline
    pipeline.fit(X_train, y_train)

    # Evaluate
    y_pred = pipeline.predict(X_test)
    overall_accuracy = accuracy_score(y_test, y_pred)

    # Per-category metrics
    per_category_metrics = {}
    categories_list = label_encoder.classes_

    for idx, cat_name in enumerate(categories_list):
        cat_mask = y_test == idx
        if cat_mask.sum() > 0:
            cat_accuracy = accuracy_score(y_test[cat_mask], y_pred[cat_mask])
            cat_precision = precision_score(y_test, y_pred, labels=[idx], average=None, zero_division=0)[0]
            cat_recall = recall_score(y_test, y_pred, labels=[idx], average=None, zero_division=0)[0]
            per_category_metrics[cat_name] = {
                'accuracy': cat_accuracy,
                'precision': cat_precision,
                'recall': cat_recall
            }

    # Category distribution in training set
    category_distribution = {}
    for idx, cat_name in enumerate(categories_list):
        count = (encoded_targets == idx).sum()
        category_distribution[cat_name] = count

    metrics = {
        'overall_accuracy': overall_accuracy,
        'per_category_metrics': per_category_metrics,
        'category_distribution': category_distribution,
    }

    return pipeline, label_encoder, metrics


def export_to_onnx(pipeline: Pipeline, label_encoder: LabelEncoder, export_path: Path) -> None:
    """
    Export scikit-learn pipeline to ONNX format.

    ONNX model includes both the vectorizer and classifier in a single inference graph.

    Args:
        pipeline: Fitted scikit-learn Pipeline
        label_encoder: Fitted LabelEncoder (for category names)
        export_path: Output path for ONNX model

    Raises:
        Exception: If ONNX export fails
    """
    # Ensure export directory exists
    export_path.parent.mkdir(parents=True, exist_ok=True)

    # Get the number of classes
    n_classes = len(label_encoder.classes_)

    # Define initial types: input is a string (merchant label)
    initial_types = [('input', StringTensorType([None]))]

    # Convert to ONNX
    try:
        onnx_model = skl2onnx.convert_sklearn(pipeline, initial_types=initial_types, target_opset=12)
        onnx.checker.check_model(onnx_model)
        onnx.save_model(onnx_model, str(export_path))
    except Exception as e:
        raise Exception(f"Failed to export ONNX model: {e}")


def print_training_report(
    csv_path: Path,
    metrics: Dict,
    label_encoder: LabelEncoder,
    export_path: Path,
    export_time_ms: float,
    file_size_kb: float
) -> None:
    """
    Print human-readable training report to stdout.

    Args:
        csv_path: Path to training data CSV
        metrics: Metrics dictionary from training
        label_encoder: Fitted LabelEncoder
        export_path: Output path for ONNX model
        export_time_ms: Time taken to export (milliseconds)
        file_size_kb: Size of exported ONNX file (kilobytes)
    """
    print("\n" + "=" * 60)
    print("Training Report: " + csv_path.name)
    print("=" * 60)

    print(f"\nOverall Accuracy: {metrics['overall_accuracy']*100:.1f}%\n")

    print("Per-Category Metrics:")
    for cat_name, cat_metrics in metrics['per_category_metrics'].items():
        accuracy = cat_metrics['accuracy'] * 100
        precision = cat_metrics['precision'] * 100
        recall = cat_metrics['recall'] * 100
        print(f"  {cat_name:15} Accuracy: {accuracy:5.1f}%, "
              f"Precision: {precision:5.1f}%, Recall: {recall:5.1f}%")

    print("\nCategory Distribution:")
    for cat_name, count in metrics['category_distribution'].items():
        print(f"  {cat_name:15} ({count:4d} transactions)")

    print(f"\nModel exported: {export_path.name} ({file_size_kb:.0f} KB, {export_time_ms:.0f} ms)")
    print("=" * 60 + "\n")


def main():
    parser = argparse.ArgumentParser(
        description="Train famfin ML categorization model"
    )
    parser.add_argument(
        "--export",
        type=Path,
        required=True,
        help="Path to export ONNX model (e.g., models/model.onnx)"
    )
    parser.add_argument(
        "--data",
        type=Path,
        default=Path("data/transactions.csv"),
        help="Path to training data CSV (default: data/transactions.csv)"
    )
    args = parser.parse_args()

    try:
        print(f"Training categorization model...")
        print(f"Data: {args.data}")
        print(f"Export: {args.export}")

        # Load training data
        print("Loading training data...")
        merchant_labels, categories = load_training_data(args.data)
        print(f"  Loaded {len(merchant_labels)} transactions, {len(set(categories))} categories")

        # Train model
        print("Training classifier...")
        start_train = time.time()
        pipeline, label_encoder, metrics = train_model(merchant_labels, categories)
        train_time = (time.time() - start_train) * 1000  # ms
        print(f"  Training complete in {train_time:.0f} ms")

        # Export to ONNX
        print("Exporting to ONNX...")
        start_export = time.time()
        export_to_onnx(pipeline, label_encoder, args.export)
        export_time = (time.time() - start_export) * 1000  # ms

        # Get file size
        file_size_kb = args.export.stat().st_size / 1024

        # Print report
        print_training_report(
            args.data,
            metrics,
            label_encoder,
            args.export,
            export_time,
            file_size_kb
        )

        print("✓ Training pipeline complete!")
        return 0

    except FileNotFoundError as e:
        print(f"✗ Error: {e}", file=sys.stderr)
        print("  Run: python scripts/train.py --help for usage", file=sys.stderr)
        return 1
    except ValueError as e:
        print(f"✗ Data error: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"✗ Training failed: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
