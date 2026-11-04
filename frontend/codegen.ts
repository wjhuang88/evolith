import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: '../docs/reference/graphql/schema.graphql',
  documents: ['src/graphql/operations/**/*.graphql'],
  ignoreNoDocuments: false,
  generates: {
    'src/graphql/generated/': {
      preset: 'client',
      presetConfig: {
        gqlTagName: 'graphql',
      },
      config: {
        useTypeImports: true,
      },
    },
  },
};

export default config;
