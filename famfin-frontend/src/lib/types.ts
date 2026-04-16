export interface AuthState {
  session_id: string
  household_id: string
  expires_at: string
}

export interface Transaction {
  id: string
  date: string
  amount: number
  merchant_name: string
  category_id?: string
  description?: string
  category_source: string
  one_time_flag: boolean
  created_at: string
  updated_at: string
}

export interface Category {
  id: string
  name: string
  color: string
  icon: string
  created_at: string
}

export interface Goal {
  id: string
  name: string
  description?: string
  target_amount: number
  current_amount: number
  deadline?: string
  generates_income: boolean
  creates_expenses: boolean
  created_at: string
}

export interface MonthlySpending {
  month: string
  total_spending: number
  category_breakdown: CategorySpending[]
}

export interface CategorySpending {
  category_id: string
  category_name: string
  amount: number
  percentage: number
}

export interface ImportResponse {
  imported_count: number
  skipped_count: number
  errors: string[]
}
