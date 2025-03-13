/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html", "../../templates/**/*.html"],
  darkMode: "class",
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography")],
};
