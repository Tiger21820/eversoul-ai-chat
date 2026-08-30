import { copyFile, mkdir, rm, stat } from 'node:fs/promises';
import { constants } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const portableDir = path.join(rootDir, 'build');
const releaseExePath = path.join(
  rootDir,
  'src-tauri',
  'target',
  'release',
  'eversoul-ai-chat.exe',
);
const portableExePath = path.join(portableDir, 'eversoul-ai-chat.exe');
const portableModelDir = path.join(portableDir, 'ai', 'model');

async function ensureFile(filePath, label) {
  const entry = await stat(filePath).catch(() => null);
  if (!entry?.isFile()) {
    throw new Error(`${label} 파일을 찾을 수 없습니다: ${filePath}`);
  }
}

await ensureFile(releaseExePath, 'Tauri release exe');

await rm(portableDir, { recursive: true, force: true });
await mkdir(portableDir, { recursive: true });
await copyFile(releaseExePath, portableExePath, constants.COPYFILE_FICLONE);
await mkdir(portableModelDir, { recursive: true });

console.info(`포터블 빌드 생성 완료: ${portableDir}`);
console.info(`- exe: ${portableExePath}`);
console.info(`- model folder: ${portableModelDir}`);
