import { create } from 'zustand'
import type { Skill, Memory, SoulFile } from '../types'

interface AppState {
  skills: Skill[];
  memories: Memory[];
  soulFiles: SoulFile[];
  dataDir: string;
  
  setSkills: (skills: Skill[]) => void;
  setMemories: (memories: Memory[]) => void;
  setSoulFiles: (files: SoulFile[]) => void;
  setDataDir: (dir: string) => void;
  
  addSkill: (skill: Skill) => void;
  updateSkill: (id: string, skill: Partial<Skill>) => void;
  removeSkill: (id: string) => void;
  
  addMemory: (memory: Memory) => void;
}

export const useAppStore = create<AppState>((set) => ({
  skills: [],
  memories: [],
  soulFiles: [],
  dataDir: '',
  
  setSkills: (skills) => set({ skills }),
  setMemories: (memories) => set({ memories }),
  setSoulFiles: (files) => set({ soulFiles: files }),
  setDataDir: (dir) => set({ dataDir: dir }),
  
  addSkill: (skill) => set((state) => ({ skills: [...state.skills, skill] })),
  updateSkill: (id, updates) => set((state) => ({
    skills: state.skills.map(s => s.id === id ? { ...s, ...updates } : s)
  })),
  removeSkill: (id) => set((state) => ({
    skills: state.skills.filter(s => s.id !== id)
  })),
  
  addMemory: (memory) => set((state) => ({ memories: [...state.memories, memory] })),
}))
