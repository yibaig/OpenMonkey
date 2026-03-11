pub const SKILL_TRANSLATION_PROMPT: &str = r#"You are a skill translator. Convert the following skill description into OpenMonkey native format (OM-SKILL.md).

Output format:
```yaml
name: <skill name>
description: <brief description>
triggers:
  - <keyword1>
  - <keyword2>
instructions: |
  <detailed instructions>
tools_required:
  - <tool1>
  - <tool2>
examples:
  - <example1>
  - <example2>
```

Input skill description:
{input}
"#;
