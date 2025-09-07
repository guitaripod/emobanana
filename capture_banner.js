const puppeteer = require('puppeteer');
const path = require('path');

(async () => {
    try {
        const browser = await puppeteer.launch({
            headless: 'new',
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        
        const page = await browser.newPage();
        await page.setViewport({ width: 560, height: 280 });
        
        const filePath = 'file://' + path.resolve(__dirname, 'banner.html');
        await page.goto(filePath, { waitUntil: 'networkidle0' });
        
        await page.screenshot({
            path: 'emobanana_banner_560x280.png',
            clip: { x: 0, y: 0, width: 560, height: 280 }
        });
        
        await browser.close();
        console.log('Banner saved as emobanana_banner_560x280.png');
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
})();