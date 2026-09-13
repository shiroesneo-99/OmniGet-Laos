# OmniGet-Laos 🇱🇦

OmniGet ສະບັບພາສາລາວ: ແອັບດາວໂຫຼດວິດີໂອ, ສຽງ ແລະ ຄອສຮຽນ ຈາກ YouTube, TikTok, Facebook, Instagram, X ແລະ ອີກ 1,800+ ເວັບ, ໂດຍບໍ່ຕ້ອງໃຊ້ terminal.

ພັດທະນາຕໍ່ຈາກ [tonhowtf/omniget](https://github.com/tonhowtf/omniget) (GPL-3.0).

## ມີຫຍັງແຕກຕ່າງຈາກສະບັບຕົ້ນ

### ພາສາ ແລະ ຟອນລາວ
- ແປໜ້າຈໍແອັບເປັນ **ພາສາລາວຄົບທຸກໜ້າ** (7,209 ຂໍ້ຄວາມ).
- ຝັງຟອນ **Noto Serif Lao** ແລະ **Noto Sans Lao** ມາພ້ອມແອັບ. ຖ້າເຄື່ອງມີ **Phetsarath OT** ກໍເລືອກໃຊ້ໄດ້ (ການຕັ້ງຄ່າ → ຕົວອັກສອນ → ຕົວອັກສອນລາວ).
- ຖອດສຽງ (Whisper) ແລະ ແປຄຳບັນຍາຍ (SRT) ເລືອກພາສາລາວ, ໄທ ແລະ ຫວຽດໄດ້.

### ປັບປຸງຄວາມປອດໄພ
- **ປລັກອິນ:** ບໍ່ດາວ ຫຼື ອັບເດດເອງຕອນເປີດແອັບອີກ. ຕອນຕິດຕັ້ງຈະກວດ sha256 ກັບ GitHub ຖ້າ release ມີຄ່າ digest.
- **ຊື່ໄຟລ໌:** ບໍ່ສົ່ງຜ່ານ `cmd` ອີກ, ຈຶ່ງໃຊ້ຊື່ໄຟລ໌ແລ່ນຄຳສັ່ງບໍ່ໄດ້.
- **ການຖອນການຕິດຕັ້ງໂປຣແກຣມ:** ອ່ານຄຳສັ່ງຈາກລະບົບເອງ, ບໍ່ເຊື່ອຄ່າທີ່ສົ່ງມາຈາກໜ້າຈໍ.
- **Browser extension:** ຈັບຄູ່ໄດ້ສະເພາະຈາກ extension ແທ້, ເວັບອື່ນແຍ່ງ token ບໍ່ໄດ້.
- **HTML ຈາກຄອສ ແລະ ບັນທຶກ:** ກັ່ນຕອງດ້ວຍ DOMPurify ແລະ ເປີດໃຊ້ Content Security Policy.

## ດາວໂຫຼດ

ໄປທີ່ໜ້າ [Releases](https://github.com/shiroesneo-99/OmniGet-Laos/releases/latest) ແລ້ວດາວໄຟລ໌ `omniget_x.y.z_x64-setup.exe` (Windows).

- **ພາສາລາວ:** ຫຼັງຕິດຕັ້ງ, ໄປທີ່ **Settings → Appearance → Language → ລາວ**.
- **Windows SmartScreen:** ຖ້າມີຄຳເຕືອນ, ກົດ **More info → Run anyway** (ແອັບຍັງບໍ່ມີລາຍເຊັນດິຈິຕອນ).
- **ການອັບເດດ:** ແອັບຈະອັບເດດຈາກ repo ນີ້, ສະນັ້ນຍັງເປັນພາສາລາວຫຼັງອັບເດດ.

## Build ເອງ (Windows)

ຕ້ອງມີ: Node.js 18+, pnpm 10, Rust 1.97.0 (ຜ່ານ rustup), Visual Studio Build Tools (C++) ແລະ WebView2.

```powershell
git clone https://github.com/shiroesneo-99/OmniGet-Laos.git
cd OmniGet-Laos
pnpm install
pnpm tauri build --bundles nsis
```

ໄຟລ໌ຕິດຕັ້ງຈະຢູ່ທີ່ `src-tauri/target/release/bundle/nsis/`. ການ build ຄັ້ງທຳອິດໃຊ້ເວລາ 30–60 ນາທີ.

ຖ້າບໍ່ມີ signing key ຂອງ updater, ໃຫ້ເພີ່ມ `--config '{\"bundle\":{\"createUpdaterArtifacts\":false}}'`.

## ອອກ Release (ສຳລັບຜູ້ດູແລ)

1. ເພີ່ມ secrets ໃນ **Settings → Secrets and variables → Actions**:
   - `TAURI_SIGNING_PRIVATE_KEY`: ເນື້ອໃນຂອງໄຟລ໌ signing key
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: ລະຫັດຜ່ານຂອງ key
2. ປ່ຽນເລກເວີຊັນ: `pnpm version:set 0.9.3`, ແລ້ວ commit.
3. ສ້າງ tag ແລ້ວ push: `git tag v0.9.3 && git push origin v0.9.3`.
4. GitHub Actions ຈະ build ແລະ ສ້າງ **draft release**. ກວດເບິ່ງກ່ອນ ແລ້ວກົດ Publish.

## ຊ່ວຍກວດຄຳແປ

AI ເປັນຜູ້ແປ, ບາງປະໂຫຍກອາດຍັງບໍ່ເປັນທຳມະຊາດ. ຖ້າພົບຄຳແປທີ່ຜິດ, ແກ້ໄຂໄດ້ທີ່ [`src/lib/i18n/lo.json`](src/lib/i18n/lo.json) ແລ້ວສົ່ງ Pull Request, ຫຼື ເປີດ Issue ບອກໄດ້ເລີຍ.

## License

GPL-3.0, ຄືກັບສະບັບຕົ້ນ. ຟອນ Noto Serif Lao ແລະ Noto Sans Lao ໃຊ້ SIL Open Font License (ເບິ່ງ `static/fonts/`).
