let generateFontSize = () => {
    let fontSize = {};
    let addRange = (start, end, step) => {
        for (let i = start; i <= end; i += step) {
            fontSize[i] = i + 'px';
        }
    };
    addRange(15, 35, 1);
    addRange(40, 60, 5);
    addRange(70, 120, 10);
    return fontSize;
};

module.exports = {
  theme: {
    fontFamily: {
        display: ['Metropolis', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
        monospace: ['Courier New', 'monospace'],
        ordinary: ['Arial', 'sans-serif']
    },
    fontSize: generateFontSize(),
    extend: {}
  },
  variants: {},
  plugins: []
}
