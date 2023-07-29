const path = require("path");
const fs = require("fs");

// Load the .env file from config dir
function loadEnv(): void {
    // absolute dotenv path
    const dotenvPath: string = path.resolve(__dirname, "../config/", ".env");
    // require the dotenv file
    require("dotenv").config({ path: dotenvPath });
}

function checkIconSize(iconFilePath: string): Promise<boolean> {
    /*
    helper function to validate plugin 
    icon size i.e less than or equal to 50KB
    */
    return new Promise((resolve, reject) => {
        fs.stat(
            iconFilePath,
            (err: NodeJS.ErrnoException | null, stats: any) => {
                if (err) {
                    reject(err);
                    return;
                }
                const fileSizeInBytes: number = stats.size;
                const fileSizeInKB: number = fileSizeInBytes / 1024;
                if (fileSizeInKB <= 50) {
                    resolve(true);
                } else {
                    resolve(false);
                }
            }
        );
    });
}

module.exports = {
    loadEnv,
    checkIconSize,
};
