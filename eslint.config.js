import globals from 'globals'
import tseslint from 'typescript-eslint'
import { defineConfig } from 'eslint/config'
import react from 'eslint-plugin-react'

export default defineConfig([
	{
		files: ['**/*.{js,jsx,mjs,cjs,ts,tsx}'],
		plugins: {
			react,
		},
		languageOptions: {
			parserOptions: {
				ecmaFeatures: {
					jsx: true,
				},
			},
			globals: {
				...globals.browser,
			},
		},
		rules: {
			'react/jsx-uses-react': 'warn',
			'react/jsx-uses-vars': 'warn',
			'no-unused-vars': 'warn',
			'no-undef': 'warn',
			'quotes': [
				'warn',
				'single'
			],
			'semi': [
				'warn',
				'never'
			]
		},
	},
	{
		ignores: [
			'.build/',
			'.sst/',
			'node_modules/',
			'src-tauri/',
			'dist/',
		]
	},
	tseslint.configs.recommended,
])
