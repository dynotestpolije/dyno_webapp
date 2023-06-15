module.exports = {
  content: [
    "src/**/*.rs",
    "index.html",
    "**/*.html",
    "**/*.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Nunito', 'sans'],
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('daisyui')
  ],

  daisyui: {
    themes: ["pastel", "business"],
    darkTheme: "business"
  },
}
