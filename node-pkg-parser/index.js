import { readFile, writeFile } from "fs/promises";

async function bootstrap() {
  const file = await readFile("2.pakkit-json");
  const arr = JSON.parse(file);

  for (let el of arr) {
    // console.log(el.meta)
    // if (el.meta.name === "map_chunk") {
    //     console.log(el)

    //     // writeFile(`chunks/c.${el.data.x}.${el.data.z}.json`, JSON.stringify(el.data))
    //     writeFile(`chunks/bin/c.${el.data.x}.${el.data.z}.nbt`, Buffer.from(el.raw) )

    //     // return
    // }

    if (el.meta.name === "update_light") {
      console.log(el);

      // writeFile(`chunks/c.${el.data.x}.${el.data.z}.json`, JSON.stringify(el.data))
      writeFile(
        `chunks/bin-light/c.${el.data.chunkX}.${el.data.chunkZ}.bin`,
        Buffer.from(el.raw)
      );

    //   return
    }
  }
}
console.clear();
bootstrap();
