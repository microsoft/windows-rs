import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';

export default [
    {
        ignores: [
            '**/webpack.config.*',
            '**/eslint.*.js',
            '**/react-app-env.d.ts',
            'dist/**',
            'build/**',
            'node_modules/**',
        ]
    },
    js.configs.recommended,
    {
        files: ['**/*.js', '**/*.jsx', '**/*.ts', '**/*.tsx'],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
                ecmaFeatures: {
                    jsx: true
                }
            },
        },
        plugins: {
            '@typescript-eslint': ts,
            react,
            'react-hooks': reactHooks,
        },
        rules: {
            ...ts.configs['recommended'].rules,
            ...react.configs.recommended.rules,
            'indent': ['error', 4, { 'SwitchCase': 1 }],
            'quotes': ['error', 'single'],
            'semi': ['error', 'always'],
            'react/jsx-uses-react': 'off',
            'react/react-in-jsx-scope': 'off',
            'react-hooks/rules-of-hooks': 'error',
            'react-hooks/exhaustive-deps': 'warn',
            '@typescript-eslint/no-unused-vars': 'warn',
            'no-undef': 'off',
        },
        settings: {
            react: {
                version: 'detect',
            },
        }
    },
    {
        files: ['.eslintrc.{js,cjs}'],
        languageOptions: {
            sourceType: 'script',
        }
    },
];