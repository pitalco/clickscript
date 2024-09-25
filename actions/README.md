# @clickscript/actions

Typescript actions for the Clickscript visual programming language.

## Installation

Install via JSR:
```bash
npx jsr install @clickscript/actions
```

## Overview

This package provides various actions that can be used in the Clickscript visual programming language. It supports Node.js and Deno environments.

## Actions

- **call**: Calls an API using fetch.
- **condition**: Determines if a condition passes and runs the subsequent action if true.
- **func**: Sets a function to be run at another time.
- **kill**: Throws an error.
- **log**: Logs messages to the console.
- **repeat**: Loops through actions based on the number of children.
- **set**: Sets a variable or constant globally.
- **text**: Performs various text operations.

## Usage

Import the desired action and use it within your project like any package:

```typescript
import { call, log } from '@clickscript/actions';

log('Hello, world!');
call({ url: 'https://api.example.com/data' });
```

## Documentation

For detailed API documentation, visit the [official docs](https://jsr.io/@clickscript/actions/doc).