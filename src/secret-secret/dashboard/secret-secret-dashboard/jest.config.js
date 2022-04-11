/** @type {import('@jest/types').Config.InitialOptions} */
module.exports = {
  verbose: true,
  moduleFileExtensions: ["js", "json", "vue", "ts"],
  preset: "@vue/cli-plugin-unit-jest/presets/typescript-and-babel",
  transform: {
    "^.+\\.vue$": "vue-jest",
    "^.+\\.tsx?$": "ts-jest",
  },

  moduleNameMapper: {
    "^@/(.*)$": "<rootDir>/src/$1",
  },
  transformIgnorePatterns: [
    "node_modules/(?!(@babel)/)",
    "/node_modules/(?!(bootstrap-vue)/)",
    "/node_modules/(?!(vue)/)",
  ],
  testMatch: ["**/tests/unit/**/*.spec.(js|jsx|ts|tsx)"],
  testURL: "http://localhost/",
  testEnvironment: "jsdom",
};
