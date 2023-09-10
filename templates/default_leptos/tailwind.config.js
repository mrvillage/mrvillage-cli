/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    `./src/**/*.rs`,
  ],
  plugins: [require("@mrvillage/tailwind"), require("@tailwindcss/typography")],
};
