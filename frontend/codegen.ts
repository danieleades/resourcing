import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	overwrite: true,
	schema: 'http://localhost:8000/graphql',
	documents: ['./src/lib/graphql/**/*.graphql', './src/**/*.svelte'],
	generates: {
		'./src/lib/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true,
				exportFragmentSpreadSubTypes: true
			}
		}
	}
};

export default config;
