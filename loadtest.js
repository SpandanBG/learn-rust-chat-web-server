// Protocol Based Testing
// https://k6.io/docs/testing-guides/load-testing-websites/
import http from 'k6/http';
import { sleep, check } from 'k6';

const HOST = "https://localhost"

export const options = {
    vus: 1000,
    duration: "30s",
    insecureSkipTLSVerify: true,
}

export default function() {
    const params = {
        'sec-ch-ua': '"Chromium";v="94", "Google Chrome";v="94", ";Not A Brand";v="99"',
        'accept-encoding': 'gzip, deflate, br',
        'accept-language': 'en-GB,en;q=0.9',
    };

    // 01. Go to the homepage
    let response = http.batch([
        ['GET', `${HOST}/`, params],
        ['GET', `${HOST}/styles.css`, params],
        ['GET', `${HOST}/script.js`, params],
        ['GET', `${HOST}/favicon.ico`, params],
    ]);
    check(response, {
        'Homepage loaded': (r) => JSON.stringify(r).includes('hello!'),
    });

    sleep(4);
}