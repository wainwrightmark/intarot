import { registerPlugin } from './@capacitor/core.js';

const RateApp = registerPlugin('RateApp', {
    web: () => import('./common/web-63c8efa2.js').then(m => new m.RateAppWeb()),
});

export { RateApp };
