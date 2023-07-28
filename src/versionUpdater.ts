const fs = require("fs");

interface PluginData {
  version: string;
}

async function updatePluginVersion(updateType: string): Promise<void> {
  const pluginJsonPath: string = process.cwd() + "/plugin.json";

  fs.readFile(
    pluginJsonPath,
    "utf8",
    (err: NodeJS.ErrnoException | null, data: string) => {
      if (err) {
        console.error("Error reading plugin.json:", err);
        process.exit(1);
      }

      try {
        const pluginData: PluginData = JSON.parse(data);

        // Assuming the version is in the format 'MAJOR.MINOR.PATCH ex (1.0.0)'
        const versionParts: string[] = pluginData.version.split(".");
        const [major, minor, patch] = versionParts;

        // Update the version based on the specified type (patch, minor, major)
        switch (updateType) {
          case "patch":
            versionParts[2] = String(parseInt(patch) + 1);
            break;
          case "minor":
            versionParts[1] = String(parseInt(minor) + 1);
            versionParts[2] = "0";
            break;
          case "major":
            versionParts[0] = String(parseInt(major) + 1);
            versionParts[1] = "0";
            versionParts[2] = "0";
            break;
          default:
            console.error(
              'Invalid update type. Use "patch", "minor", or "major".'
            );
            process.exit(1);
        }
        const updatedVersion: string = versionParts.join(".");
        pluginData.version = updatedVersion;

        // Write the updated data back to the plugin.json file
        fs.writeFile(
          pluginJsonPath,
          JSON.stringify(pluginData, null, 2),
          "utf8",
          (err: NodeJS.ErrnoException | null) => {
            if (err) {
              console.error("Error writing plugin.json:", err);
              process.exit(1);
            } else {
              console.log(
                `Plugin version updated successfully to ${updatedVersion}`
              );
            }
          }
        );
      } catch (err) {
        console.error("Error parsing plugin.json:", err);
        process.exit(1);
      }
    }
  );
}

export default updatePluginVersion;
