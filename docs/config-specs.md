# Configuration File Specs

## Enviroment Setup

```bash
  # Where does the "root" config exist
  export CONFREENCE_ROOT_CONIFG="~/.confreence.config.yaml"
```

```typescript
  export type Config = {
    credentials: {
      [providerName: string]: String;
    };
    // Where the "build" starts from.
    rootDir: String;
    // List of children directories.
    childrenDir: {
      [resourceName: string]: String;
    } 
  }
```
