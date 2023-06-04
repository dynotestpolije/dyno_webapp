module.exports = {
    content: [ "./src/**/*.rs", "./index.html" ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Nunito', 'sans'],
            },
        },
        container: {
            center: true,
        }
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
        require('daisyui')
    ],

    daisyui: {
        themes: ["lofi", "black"],
    },

    extend: {
      animation: {
        cursor: 'cursor .6s linear infinite alternate',
        type: 'type 2.7s ease-out .8s infinite alternate both',
        'type-reverse': 'type 1.8s ease-out 0s infinite alternate-reverse both',
      },
      keyframes: {
        type: {
          '0%': { transform: 'translateX(0ch)' },
          '5%, 10%': { transform: 'translateX(1ch)' },
          '15%, 20%': { transform: 'translateX(2ch)' },
          '25%, 30%': { transform: 'translateX(3ch)' },
          '35%, 40%': { transform: 'translateX(4ch)' },
          '45%, 50%': { transform: 'translateX(5ch)' },
          '55%, 60%': { transform: 'translateX(6ch)' },
          '65%, 70%': { transform: 'translateX(7ch)' },
          '75%, 80%': { transform: 'translateX(8ch)' },
          '85%, 90%': { transform: 'translateX(9ch)' },
          '95%, 100%': { transform: 'translateX(11ch)' },
        },
      },
    }
}
