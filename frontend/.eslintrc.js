module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/recommended',
    // 'plugin:prettier/recommended', // Add this line if/when eslint-plugin-prettier is installed
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 12,
    sourceType: 'module',
  },
  plugins: [
    'react',
    '@typescript-eslint',
    // 'prettier', // Add this line if/when eslint-plugin-prettier is installed
  ],
  rules: {
    'react/react-in-jsx-scope': 'off', // For Vite/React 17+
    'react/prop-types': 'off', // Since we are using TypeScript for prop types
    '@typescript-eslint/explicit-module-boundary-types': 'off',
    // 'prettier/prettier': 'warn', // Add this line if/when eslint-plugin-prettier is installed
    // Add any project-specific rules here
  },
  settings: {
    react: {
      version: 'detect', // Automatically detects the React version
    },
  },
  ignorePatterns: ['dist/', 'node_modules/', 'vite.config.ts'], // Ignore build outputs and node_modules
};
