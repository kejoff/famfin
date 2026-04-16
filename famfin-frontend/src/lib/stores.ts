import { writable } from 'svelte/store'
import type { AuthState } from './types'

export const authStore = writable<AuthState | null>(null)

// Use relative path so Vite proxy works (dev) and production URL works
export const API_BASE = '/api'

export async function apiCall(
  method: string,
  endpoint: string,
  auth: AuthState | null,
  body?: any
) {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  }

  const response = await fetch(`${API_BASE}${endpoint}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
    credentials: 'include', // Include session cookie
  })

  if (!response.ok) {
    const error = await response.text()
    throw new Error(`API Error: ${response.status} - ${error}`)
  }

  return response.status === 204 ? null : response.json()
}
