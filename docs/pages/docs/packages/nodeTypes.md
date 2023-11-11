# @datagen-rs/types

The `datagen-rs-types` package provides TypeScript types for the `datagen-rs-node` package.
This package includes a TypeScript JSON schema definition as well as the JSON schema
itself. Additionally, types for
[creating node.js plugins](https://markusjx.github.io/datagen/docs/plugins/node/create/)
are included.

## Installation

```bash
npm install datagen-rs-types
```

## Provided Types

### JSON Schema

The JSON schema is provided as a TypeScript type and as a JSON file. The JSON schema file
can be imported using

```ts
import { SchemaJson } from 'datagen-rs-types';
```

The base TypeScript definition for the JSON schema is generated by
[`json-schema-to-typescript`](https://www.npmjs.com/package/json-schema-to-typescript)
and can be imported using

```ts
import { Schema } from 'datagen-rs-types';
```

### Node.js Plugin Types

The types for creating node.js plugins are also provided by this package. These types
can be used to create plugins that can be used with the `datagen-rs-node` package.
The main plugin type is `DatagenPlugin` and can be imported using

```ts
import { DatagenPlugin } from 'datagen-rs-types';
```

Check out the
[node.js plugin documentation](https://markusjx.github.io/datagen/docs/plugins/node/create/)
for more information on creating node.js plugins.