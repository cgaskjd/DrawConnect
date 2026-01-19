import { create } from 'zustand'

export type PageMode = 'landing' | 'plugins'

interface NavigationState {
  pageMode: PageMode
  setPageMode: (mode: PageMode) => void
}

export const useNavigationStore = create<NavigationState>((set) => ({
  pageMode: 'landing',
  setPageMode: (mode: PageMode) => set({ pageMode: mode }),
}))
