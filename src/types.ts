export interface Skill {
  id: string;
  name: string;
  description: string;
  triggers: string[];
  instructions: string;
  tools_required: string[];
  examples: string[];
  enabled: boolean;
}

export interface Memory {
  id: number;
  content: string;
  tags?: string;
  importance: number;
  created_at: string;
}

export interface SoulFile {
  name: string;
  content: string;
  path: string;
}
