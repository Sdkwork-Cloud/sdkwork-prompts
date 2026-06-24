# SDKWork Prompts PC

PC application for SDKWork Prompts.

## Architecture

This application follows `APP_PC_ARCHITECTURE_SPEC.md` standards.

### Package Structure

- `sdkwork-prompts-pc-core` - Core runtime, SDK clients, session management
- `sdkwork-prompts-pc-commons` - Shared UI components and utilities
- `sdkwork-prompts-pc-shell` - PC navigation and shell components
- `sdkwork-prompts-pc-discussion` - Prompts discussion feature package

### Development

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build
```

### Configuration

Runtime configuration is in `config/browser/`. See `CONFIG_SPEC.md` for details.
