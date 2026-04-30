import js from "@eslint/js";
import pluginVue from "eslint-plugin-vue";

export default [
  js.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  {
    files: ["**/*.js", "**/*.vue"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        browser: true,
        node: true,
        process: "readonly",
      },
    },
    rules: {
      "vue/multi-word-component-names": "off",
      "no-unused-vars": "warn",
      "no-console": "warn",
      "max-lines-per-function": ["error", { "max": 20, "skipBlankLines": true, "skipComments": true }],
    },
  },
  {
    ignores: ["dist/**", "src-tauri/target/**", "node_modules/**"],
  },
];
