export const editorSections = [
  { id: 'scenario', label: 'Scenario' },
  { id: 'diagnostics', label: 'Diagnostics' },
] as const;

export type EditorSectionId = (typeof editorSections)[number]['id'];
