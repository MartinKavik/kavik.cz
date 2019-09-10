// [...range(1,5,2)] => [1,3,5]
function* range(start, end, step) {
    if(start > end) return;
    yield start;
    yield* range(start + step, end, step);
};

// For NodeJS < 12
Object.fromEntries = Object.fromEntries || ((iterable) => {
  return [...iterable].reduce((obj, [key, val]) => {
    obj[key] = val
    return obj
  }, {})
});

module.exports = {
  theme: {
    fontFamily: {
        display: ['Metropolis', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
        monospace: ['Courier New', 'monospace'],
        ordinary: ['Arial', 'sans-serif'],
        sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial',
            'Noto Sans', 'sans-serif', 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji']
    },
    fontSize: (() => {
        return Object.fromEntries([
            ...range(15, 35, 1),
            ...range(40, 60, 5),
            ...range(70, 120, 10)
        ].map(i => [i, `${i}px`]))
    })(),
    colors: {
        black: "#000",
        gray: {
            1: 'hsl(210, 36%, 96%)',
            2: 'hsl(212, 33%, 89%)',
            3: 'hsl(210, 31%, 80%)',
            4: 'hsl(211, 27%, 70%)',
            5: 'hsl(209, 23%, 60%)',
            6: 'hsl(210, 22%, 49%)',
            7: 'hsl(209, 28%, 39%)',
            8: 'hsl(209, 34%, 30%)',
            9: 'hsl(211, 39%, 23%)',
            10: 'hsl(209, 61%, 16%)',
        },
        blue: {
            1: 'hsl(205, 79%, 92%)',
            2: 'hsl(205, 97%, 85%)',
            3: 'hsl(205, 84%, 74%)',
            4: 'hsl(205, 74%, 65%)',
            5: 'hsl(205, 65%, 55%)',
            6: 'hsl(205, 67%, 45%)',
            7: 'hsl(205, 76%, 39%)',
            8: 'hsl(205, 82%, 33%)',
            9: 'hsl(205, 87%, 29%)',
            10: 'hsl(205, 100%, 21%)',
        },
        yellow: {
            1: 'hsl(49, 100%, 96%)',
            2: 'hsl(48, 100%, 88%)',
            3: 'hsl(48, 95%, 76%)',
            4: 'hsl(48, 94%, 68%)',
            5: 'hsl(44, 92%, 63%)',
            6: 'hsl(42, 87%, 55%)',
            7: 'hsl(36, 77%, 49%)',
            8: 'hsl(29, 80%, 44%)',
            9: 'hsl(22, 82%, 39%)',
            10: 'hsl(15, 86%, 30%)',
        },
    },
    opacity: (() => {
        return Object.fromEntries([
            ...range(0, 100, 10),
        ].map(i => [i, i / 100]))
    })(),
    extend: {
        margin: {
            '-10vh': '-10vh',
            '-260px': '-260px',
            '-545px': '-545px',
        },
        inset: {
          '1/2': '50%',
          'full': '100%',
          '-50vh': '-50vh',
        },
        width: {
          '76': '19rem',
          'xs': '20rem',
          'md': '28rem',
          '265px': '265px',
          '570px': '570px',
          '1090px': '1090px',
          '50vh': '50vh',
        },
        height: {
          '300px': '300px',
          '320px': '320px',
          '570px': '570px',
          '580px': '580px',
          '690px': '690px',
          '1090px': '1090px',
          '1160px': '1160px',
          '1480px': '1480px',
          '1580px': '1580px',
        },
        borderRadius: {
          '110px': '110px',
          '140px': '140px'
        }
    }
  },
  variants: {},
  plugins: []
}
