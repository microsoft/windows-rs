import * as ReactDOM from 'react-dom/client';
import { FluentProvider, createLightTheme } from '@fluentui/react-components';
import '@fontsource/fira-sans';
import App from './app';

const rustVariants = {
    10: '#050201',
    20: '#22130A',
    30: '#3A1D11',
    40: '#4E2514',
    50: '#632D16',
    60: '#793518',
    70: '#8F3C19',
    80: '#A74319',
    90: '#B75227',
    100: '#C0653D',
    110: '#CA7753',
    120: '#D28969',
    130: '#DB9B7F',
    140: '#E3AD95',
    150: '#EABFAC',
    160: '#F1D1C3',
};

const coreTheme = {
    fontFamilyBase: 'Fira Sans',
    colorBrandForeground2: rustVariants[110],
};

const lightTheme = {
    ...createLightTheme(rustVariants),
    ...coreTheme,
};

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
root.render(
    // https://github.com/microsoft/fluentui/issues/30450
    //<StrictMode>
    <FluentProvider theme={lightTheme}>
        <App />
    </FluentProvider>
    //</StrictMode>
);
