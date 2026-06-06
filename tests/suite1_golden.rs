//! Suite 1 — Golden Tests (Mode A)
//! Source of truth: Dhad-Spec-v1.0 JSON test vectors
//! GT-092/093/094/095 excluded: MADD bits impossible in Mode A → moved to suite2

use dhad::modes::process_mode_a;

/// Macro: test stream bytes + both hashes exactly.
macro_rules! golden {
    ($name:ident, $input:expr, $stream:expr, $core:expr, $phonetic:expr) => {
        #[test]
        fn $name() {
            let result =
                process_mode_a($input).unwrap_or_else(|e| panic!("unexpected error: {:?}", e));
            assert_eq!(
                result.stream.to_bytes().as_slice(),
                $stream,
                "atom stream mismatch in {}",
                stringify!($name)
            );
            assert_eq!(
                hex::encode(result.core_hash),
                $core,
                "CoreHash mismatch in {}",
                stringify!($name)
            );
            assert_eq!(
                hex::encode(result.phonetic_hash),
                $phonetic,
                "PhoneticHash mismatch in {}",
                stringify!($name)
            );
        }
    };
}

// ═══════════════════════════════════════════════════════════════════
// GT-117: Empty input → empty stream (MANDATORY ANCHOR)
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_117_empty,
    b"",
    b"",
    "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0",
    "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7"
);

// ═══════════════════════════════════════════════════════════════════
// GT-001 إلى GT-028: الحروف الـ28 (Core Letters, bare)
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_001_alef,
    &[0xD8, 0xA7],
    &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
    "984a596fe5175c6413a180a8d1f09891fb53675f5a8b9daac5a1dd4a2ea784d0"
);
golden!(
    gt_002_beh,
    &[0xD8, 0xA8],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_003_teh,
    &[0xD8, 0xAA],
    &[0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "b7406c7f935e0b48f468f656c4ef9104e8c4ea1f772ddd2efc71ccc4cf8cd7f6",
    "e7d29979c8bd4ac5e64f0a760b660ddfc5b7cbcb646fcf50847fa58fa524f507"
);
golden!(
    gt_004_theh,
    &[0xD8, 0xAB],
    &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "00f3cb11ae7a5e3939867c1c81cb52eae7fe03f51c35b5296e010c5cf9639660",
    "874fe7b9c2b4d9952400eb5eb7f8e6fe6afb040713d77bf62e613bbe4ca2f294"
);
golden!(
    gt_005_jeem,
    &[0xD8, 0xAC],
    &[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "d69525af402651c93fa677d095020f6c63bd5ccab9fe160422d64e027eb1307e",
    "a9526cd2a71b0f8519f15cf3a17cd87e1ccd4eaf8d54a5d2e5b93d637741574d"
);
golden!(
    gt_006_hah,
    &[0xD8, 0xAD],
    &[0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "1e009471fe0d6fbe92bc7dbb57c761e99eb784c6f8560800e3784387de65a349",
    "cab4780f10bf9f839338c66c1c559aa1318b54a37a65259f4bf21b5990a17880"
);
golden!(
    gt_007_khah,
    &[0xD8, 0xAE],
    &[0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "00ccda52f7145871ef7cf385735caeb698bdd1ed7fcbdac02243d267bb42499f",
    "8ca9e7edbfc84750ed139ba12faed2856e5c24321cf2b87b05932ada594e35c7"
);
golden!(
    gt_008_dal,
    &[0xD8, 0xAF],
    &[0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "0f3eeae6e8f1b4038fc9abbca55a0361315bc90e72d1abb506bccb33decfa7c7",
    "66cb6563624ca7271c3b688ed5f49ac5787f2d0e13806444526034f679ec1703"
);
golden!(
    gt_009_thal,
    &[0xD8, 0xB0],
    &[0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "a90a25b688af701c61e83a648dcc371f4193f72874b5731893c50e74e9485476",
    "75669dd858243389dbb6f09eb78db84b71251fd04448689acbfc0609bf863528"
);
golden!(
    gt_010_reh,
    &[0xD8, 0xB1],
    &[0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "1abe4c2dc3142434a9f75d9de2561f42f6dc6f087ccdaf9fbd6c6b189366fcd5",
    "763e789a85528db4f843cbaa4369fec45f832ce5857aae4212607dbf52aaef4d"
);
golden!(
    gt_011_zain,
    &[0xD8, 0xB2],
    &[0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "9158bc22b5b8f78c4b8c8e214b41ba745d619e38224f9f5042c3d24ab99aac46",
    "33573247ead48a95fa0459597e9e04c0d2e1dbc4a608e48eaf1eeff4bc0d9b0d"
);
golden!(
    gt_012_seen,
    &[0xD8, 0xB3],
    &[0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "0a27952dbbe6b788c19077c600b10527e5bd8a58768ebdf751917dad60e1aad8",
    "c8a412b21764893e1ebed7d49dc130990198cf833a2c49fbaf2891d882de151c"
);
golden!(
    gt_013_sheen,
    &[0xD8, 0xB4],
    &[0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "c26812351efe98b4ccd166a99d3e154e9fdcb81f2b65cbb179d9e5311b5c2eff",
    "9ee759203706c262cd9a88a75d7d15cdbd2015973ad823c21c2a2f6391541f64"
);
golden!(
    gt_014_sad,
    &[0xD8, 0xB5],
    &[0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ac1d71738165ecc180191f0e17b13fa001bf8a044aba46367166f47c2ba2bb1e",
    "0672118f4a8fca3ae4f8501257fc9bfe70f2acca83084386bd7a3f0b2ec0800e"
);
golden!(
    gt_015_dad,
    &[0xD8, 0xB6],
    &[0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "c61c835bd964bd0d6ae6b0cf27aee404e5288d6b1dc42db8a0e7a5832ed3875a",
    "10146ac1629602bdaf031a19619cd90ac66195cc4ec75beb6252bf5441962f52"
);
golden!(
    gt_016_tah,
    &[0xD8, 0xB7],
    &[0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "f5b0bea8b8549fd811e2e4d106dfbb84beb869f9b30853e8fca0460fc399144a",
    "d312ea60d6d7a3992ffd05dbaa1fdf9165299a4473c66e09ec1645a124a8d8b0"
);
golden!(
    gt_017_zah,
    &[0xD8, 0xB8],
    &[0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "0efccc5867d9acc632683458a064bbf1f7725a180b7bc835254f897761534e32",
    "5cc2477bf62b4dd33ab921e458fd0812dddc2ffd4606b01c6424c0f8086bfc4a"
);
golden!(
    gt_018_ain,
    &[0xD8, 0xB9],
    &[0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "087119057893476200afa2fd45d4360e73938ba3c70ea203259c79299c8f34fd",
    "36f5b31aa81ccd8cb4749f5dfea255243a18c2f0633797ed057807a1a72621b1"
);
golden!(
    gt_019_ghain,
    &[0xD8, 0xBA],
    &[0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "327e61c10617b1bf1373206aefe45d8f65b608a843c20f0064a88b71e21ce459",
    "1dd651be0224b187e42bee33cba29b86a70ed09e9b968a18e20e468b87f080df"
);
golden!(
    gt_020_feh,
    &[0xD9, 0x81],
    &[0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "bc2d6a831b01c132dc5bb762a99d72612d9eb18c4f9534e92b91192f8771f6c5",
    "c02be8c53adba7aebb0accdf3c2059893cce4bed2acdb299a42073a6524ef210"
);
golden!(
    gt_021_qaf,
    &[0xD9, 0x82],
    &[0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "d196beaf118b0c16c696a03cbed73cdc24c77650d368d1dadb1365b0eb008693",
    "a84817f0b1824d7b3eab038ef55fb90efd8fa6c838cd4682746ddd082344ced7"
);
golden!(
    gt_022_kaf,
    &[0xD9, 0x83],
    &[0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "f0eadc3e579a345aed2466ae6a76ce45f51f75c935bca8c2abe5b320ba76cbf9",
    "530cceea936fca7deed1af342dde53d772f0c8403c77c72bd179a00c4309ab91"
);
golden!(
    gt_023_lam,
    &[0xD9, 0x84],
    &[0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "9cd171d8b0a55e72f36a50db8b2985fa214a02cceba34e8ade177ac857d67451",
    "af98c451e60da72a91c023ecbd946f70101fbbf6e70c6723c1aee80aa74413d1"
);
golden!(
    gt_024_meem,
    &[0xD9, 0x85],
    &[0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "8f3d77394ccf902c6f70df2ad941efdb5fba255baec24cd27712be7757a4114e",
    "3877a876454e0dcec25c6778afe6505e49ded36415af9d6d279c9bd2c98b3a0b"
);
golden!(
    gt_025_noon,
    &[0xD9, 0x86],
    &[0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4163ae2243aed7a756f03504fc966e69677299245331d6381be8c45b60511019",
    "5dc0e3712d074f51db1c7c41a7c08a7efc320e2523ca294563ffeec0b92ccb19"
);
golden!(
    gt_026_heh,
    &[0xD9, 0x87],
    &[0x1A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "a20e9ce20a3a68fe0a3e3cffd25d3e2bc484afc8f558e60466fa38e8462d7a1d",
    "c7924361547fa16601fd04f58aafc7669e3afc0fc4559682cb22025893f2758a"
);
golden!(
    gt_027_waw,
    &[0xD9, 0x88],
    &[0x1B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a",
    "41a95e6dae78877b0928ff5172bfb34482bc17824c6ab1768a8dfd74a44daefb"
);
golden!(
    gt_028_yeh,
    &[0xD9, 0x8A],
    &[0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "8aac943540c928674e5b2e38ef9f89c08b80117c051921f7cd83ef933d5d62f8",
    "4102f81b644d8e0a1b33d0fcb00addafdec8e034e45b6ba2f2b85055afbf1475"
);

// ═══════════════════════════════════════════════════════════════════
// GT-029 إلى GT-032: الوحدات الإملائية
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_029_hamza_standalone,
    &[0xD8, 0xA1],
    &[0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "40ab5329ead27ccb742144a6a5e65a19ce73290b4611775cfb8c121030e94f4a",
    "0bf13928c3674a8a21e9ebbae568504aae7b5eab9216c18e88f2fd025f199510"
);
golden!(
    gt_030_teh_marbuta,
    &[0xD8, 0xA9],
    &[0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "84f3c632ad9e86ad604f691bd4fd4093fae9dab343944c1eef5797d0a1f702a5",
    "a56fb07d894f3fa6d260045cf34071e5e6927e6e09986a072c3b881a2bea3d70"
);
golden!(
    gt_031_alef_maqsura,
    &[0xD9, 0x89],
    &[0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "cbd7e522708e96087330790a3e2beb03115e545e8788400853c4056450c411cb",
    "5c5bd83ca56a56bb940b577afc28d94f1d8c00e7332888cbfe8497f1825d69c9"
);
golden!(
    gt_032_alef_wasla,
    &[0xD9, 0xB1],
    &[0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "3f6607dd133e0f7acd1433ad43ec716933ffe6d9ca9a1600dd9b600bc773625b",
    "9a5f48d509507b5867648912b79be2f620687fbf342c111bf757e5e1bab5e867"
);

// ═══════════════════════════════════════════════════════════════════
// GT-033 إلى GT-038: الأحرف الهيكلية
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_033_space,
    &[0x20],
    &[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ca97a20db16248f8b6568c5ee06ce360103e625b6d78041aa0ede08291500b8a",
    "9b45dad7ecb1006c7d340b506b471be0676db167eb39645d2d15db453d7bd736"
);
golden!(
    gt_034_arabic_comma,
    &[0xD8, 0x8C],
    &[0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "dc9f7c6c8bfb9aaf17972a57e3779a5c9641b5e69b2dd86753e9064c39d481f7",
    "55d12a5157174d02fcce22653122ade471f917201e7e07505acab777d760e102"
);
golden!(
    gt_035_arabic_semicolon,
    &[0xD8, 0x9B],
    &[0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "0bb24516af153ede71049988524a7b475304e33dcb9596dbd3283543de090e37",
    "87bba06fd8f6d3c4211a6134a83ee2ca626b04eabb21966bdf357a0ec592fbfe"
);
golden!(
    gt_036_arabic_question,
    &[0xD8, 0x9F],
    &[0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "f81df8a526c33f2460563a339e69e7e5fe1b616f4b5eea854ac41bae77d126da",
    "40e6cbb409e15dfa36e2500d8e6b78cd0b32577a16cf972bed7e910b0c6f63b9"
);
golden!(
    gt_037_full_stop,
    &[0x2E],
    &[0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "c34769c6cd4bd0221b0c2ef3c28ded9950fdd07d6694bfd64ab6637381ef5b75",
    "98336e100b5095fc414b66dbc6c2580c6b1e7ff2c7c993f6e638d6b687fe0caf"
);
golden!(
    gt_038_colon,
    &[0x3A],
    &[0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "5fd4aaa4ccedbc06b519e36afcb89475641db1109992d1ae81ee72b2545b4ba3",
    "6ebcd52298ad5e8004bb9f43ce902e554749d5cdf61bc266366791595d055ec0"
);

// ═══════════════════════════════════════════════════════════════════
// GT-039 إلى GT-068: الأرقام (ثلاثة أشكال × 10 أرقام)
// ═══════════════════════════════════════════════════════════════════

// الرقم 0
golden!(
    gt_039_digit0_ascii,
    &[0x30],
    &[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4f93316d410538e4cfaca79d3858b4b968911dcf1c1547fad15a58a7fd74b319",
    "fa358a6b96d1f61f69005c6818bb73585eaaf9361ad1aba8aa7dbaf0c26f9f17"
);
golden!(
    gt_040_digit0_arabic,
    &[0xD9, 0xA0],
    &[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4f93316d410538e4cfaca79d3858b4b968911dcf1c1547fad15a58a7fd74b319",
    "fa358a6b96d1f61f69005c6818bb73585eaaf9361ad1aba8aa7dbaf0c26f9f17"
);
golden!(
    gt_041_digit0_extended,
    &[0xDB, 0xB0],
    &[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4f93316d410538e4cfaca79d3858b4b968911dcf1c1547fad15a58a7fd74b319",
    "fa358a6b96d1f61f69005c6818bb73585eaaf9361ad1aba8aa7dbaf0c26f9f17"
);

// الرقم 1
golden!(
    gt_042_digit1_ascii,
    &[0x31],
    &[0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "1bb193bf83dbfcd2df724b5be0221c0ed8c702d24fc5e6be09addce53cc91b87",
    "f5935476ff704c7331bbbbbba15b0e3f873cde55faa1fa39d89c747ebad1e5a9"
);
golden!(
    gt_043_digit1_arabic,
    &[0xD9, 0xA1],
    &[0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "1bb193bf83dbfcd2df724b5be0221c0ed8c702d24fc5e6be09addce53cc91b87",
    "f5935476ff704c7331bbbbbba15b0e3f873cde55faa1fa39d89c747ebad1e5a9"
);
golden!(
    gt_044_digit1_extended,
    &[0xDB, 0xB1],
    &[0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "1bb193bf83dbfcd2df724b5be0221c0ed8c702d24fc5e6be09addce53cc91b87",
    "f5935476ff704c7331bbbbbba15b0e3f873cde55faa1fa39d89c747ebad1e5a9"
);

// الرقم 2
golden!(
    gt_045_digit2_ascii,
    &[0x32],
    &[0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "da3d586554e23c32bf2522c9ff591002a39604c7bbe3e1021c960fe142895ca9",
    "bd4ab2b713ac46e2bf8921ada9004909a5202818da5643eacf4bae0f134f3bd8"
);
golden!(
    gt_046_digit2_arabic,
    &[0xD9, 0xA2],
    &[0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "da3d586554e23c32bf2522c9ff591002a39604c7bbe3e1021c960fe142895ca9",
    "bd4ab2b713ac46e2bf8921ada9004909a5202818da5643eacf4bae0f134f3bd8"
);
golden!(
    gt_047_digit2_extended,
    &[0xDB, 0xB2],
    &[0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "da3d586554e23c32bf2522c9ff591002a39604c7bbe3e1021c960fe142895ca9",
    "bd4ab2b713ac46e2bf8921ada9004909a5202818da5643eacf4bae0f134f3bd8"
);

// الرقم 3
golden!(
    gt_048_digit3_ascii,
    &[0x33],
    &[0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "7e00916713a52523d4500f2cdc174b33949a2073de752e1169ab0dc8e7256a72",
    "c3c36f616072455b1c67c659de5333a32e87b6c8792d88a4522890dd3bbf071c"
);
golden!(
    gt_049_digit3_arabic,
    &[0xD9, 0xA3],
    &[0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "7e00916713a52523d4500f2cdc174b33949a2073de752e1169ab0dc8e7256a72",
    "c3c36f616072455b1c67c659de5333a32e87b6c8792d88a4522890dd3bbf071c"
);
golden!(
    gt_050_digit3_extended,
    &[0xDB, 0xB3],
    &[0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "7e00916713a52523d4500f2cdc174b33949a2073de752e1169ab0dc8e7256a72",
    "c3c36f616072455b1c67c659de5333a32e87b6c8792d88a4522890dd3bbf071c"
);

// الرقم 4
golden!(
    gt_051_digit4_ascii,
    &[0x34],
    &[0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "40f241863eeceb61fdd699eb75a15ec5d2bc27ae21fa46fcc9687bcb18cc9138",
    "e4429e147ffad2a1613eb1a60a1631ae5d597f817a1388c15db6f6b95dc2c4ab"
);
golden!(
    gt_052_digit4_arabic,
    &[0xD9, 0xA4],
    &[0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "40f241863eeceb61fdd699eb75a15ec5d2bc27ae21fa46fcc9687bcb18cc9138",
    "e4429e147ffad2a1613eb1a60a1631ae5d597f817a1388c15db6f6b95dc2c4ab"
);
golden!(
    gt_053_digit4_extended,
    &[0xDB, 0xB4],
    &[0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "40f241863eeceb61fdd699eb75a15ec5d2bc27ae21fa46fcc9687bcb18cc9138",
    "e4429e147ffad2a1613eb1a60a1631ae5d597f817a1388c15db6f6b95dc2c4ab"
);

// الرقم 5
golden!(
    gt_054_digit5_ascii,
    &[0x35],
    &[0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "de0abdb12eda178b594dddca3646466589a24f95799e4fe839b154a9c0a407d5",
    "deaf662373337617d58ba004db0bacd53017c7c7f430f3db2523937be52dac72"
);
golden!(
    gt_055_digit5_arabic,
    &[0xD9, 0xA5],
    &[0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "de0abdb12eda178b594dddca3646466589a24f95799e4fe839b154a9c0a407d5",
    "deaf662373337617d58ba004db0bacd53017c7c7f430f3db2523937be52dac72"
);
golden!(
    gt_056_digit5_extended,
    &[0xDB, 0xB5],
    &[0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "de0abdb12eda178b594dddca3646466589a24f95799e4fe839b154a9c0a407d5",
    "deaf662373337617d58ba004db0bacd53017c7c7f430f3db2523937be52dac72"
);

// الرقم 6
golden!(
    gt_057_digit6_ascii,
    &[0x36],
    &[0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "b8b87face1ef3f46f55189aa1721bae069c037cf464391e587f209e2ac44ddd3",
    "71ff5f8704b4941167f0ea7e24c4b60a067c40badc0871318d25032911bc800e"
);
golden!(
    gt_058_digit6_arabic,
    &[0xD9, 0xA6],
    &[0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "b8b87face1ef3f46f55189aa1721bae069c037cf464391e587f209e2ac44ddd3",
    "71ff5f8704b4941167f0ea7e24c4b60a067c40badc0871318d25032911bc800e"
);
golden!(
    gt_059_digit6_extended,
    &[0xDB, 0xB6],
    &[0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "b8b87face1ef3f46f55189aa1721bae069c037cf464391e587f209e2ac44ddd3",
    "71ff5f8704b4941167f0ea7e24c4b60a067c40badc0871318d25032911bc800e"
);

// الرقم 7
golden!(
    gt_060_digit7_ascii,
    &[0x37],
    &[0x07, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "87513251a25a7547e406c31a90d2b1e796771bbd9daae7d86ce583a84d3c9934",
    "dbf27baae20b627568b81bdf7169c8fab33a350848dfdfba4081697a2603276a"
);
golden!(
    gt_061_digit7_arabic,
    &[0xD9, 0xA7],
    &[0x07, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "87513251a25a7547e406c31a90d2b1e796771bbd9daae7d86ce583a84d3c9934",
    "dbf27baae20b627568b81bdf7169c8fab33a350848dfdfba4081697a2603276a"
);
golden!(
    gt_062_digit7_extended,
    &[0xDB, 0xB7],
    &[0x07, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "87513251a25a7547e406c31a90d2b1e796771bbd9daae7d86ce583a84d3c9934",
    "dbf27baae20b627568b81bdf7169c8fab33a350848dfdfba4081697a2603276a"
);

// الرقم 8
golden!(
    gt_063_digit8_ascii,
    &[0x38],
    &[0x08, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ff3b3bf1ba0a922c06243c6bae0c70cd223a7b19742dfabb29495169e20edf4e",
    "bb22a1997a78e46552210323a9cef3a607f6e49b7c86d37977756ffd069a1a59"
);
golden!(
    gt_064_digit8_arabic,
    &[0xD9, 0xA8],
    &[0x08, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ff3b3bf1ba0a922c06243c6bae0c70cd223a7b19742dfabb29495169e20edf4e",
    "bb22a1997a78e46552210323a9cef3a607f6e49b7c86d37977756ffd069a1a59"
);
golden!(
    gt_065_digit8_extended,
    &[0xDB, 0xB8],
    &[0x08, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ff3b3bf1ba0a922c06243c6bae0c70cd223a7b19742dfabb29495169e20edf4e",
    "bb22a1997a78e46552210323a9cef3a607f6e49b7c86d37977756ffd069a1a59"
);

// الرقم 9
golden!(
    gt_066_digit9_ascii,
    &[0x39],
    &[0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "3573b8843646a901e90000b34fa187b3a4cb8f84d1debdb021278f542de5bc8b",
    "7f8788346a8e21162b55242f7fc8aa6b30efaabe0634c3b65556522b3017c732"
);
golden!(
    gt_067_digit9_arabic,
    &[0xD9, 0xA9],
    &[0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "3573b8843646a901e90000b34fa187b3a4cb8f84d1debdb021278f542de5bc8b",
    "7f8788346a8e21162b55242f7fc8aa6b30efaabe0634c3b65556522b3017c732"
);
golden!(
    gt_068_digit9_extended,
    &[0xDB, 0xB9],
    &[0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "3573b8843646a901e90000b34fa187b3a4cb8f84d1debdb021278f542de5bc8b",
    "7f8788346a8e21162b55242f7fc8aa6b30efaabe0634c3b65556522b3017c732"
);

// ═══════════════════════════════════════════════════════════════════
// GT-069 إلى GT-076: Marks
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_069_beh_fatha,
    &[0xD8, 0xA8, 0xD9, 0x8E],
    &[0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
    "f4226a79f1c62998559c44298ad718388045dc6cd5c096a9bc197175268d2a04",
    "97b6eb001215607e9e8424d96893106a648b21f345d06c8819591a481c2b6ec8"
);
golden!(
    gt_070_beh_damma,
    &[0xD8, 0xA8, 0xD9, 0x8F],
    &[0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ba64bf3ee86b4f5bfaf922913d69c78000f70f15b1b3a644ad2afc6cb57551d9",
    "cdea176dac6e5d3845e8103c53018b3e0301dc4cdb026aee318bac3edde6703b"
);
golden!(
    gt_071_beh_kasra,
    &[0xD8, 0xA8, 0xD9, 0x90],
    &[0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00],
    "5757919eb8d95a59965390027fc97477cf250cbf016922b14460da4a16e508e8",
    "7a7e4e51c108bd69a2de0842eafeed553b8bfc248ab48fb40358f23d801b0d5d"
);
golden!(
    gt_072_beh_sukun,
    &[0xD8, 0xA8, 0xD9, 0x92],
    &[0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00],
    "e3598b275d7719c77bb7e179307ce5139007bb3419aed0ddbdd52922ea156c4c",
    "2fc0b5baccab517c9b040cf890ea4ed5d6bd019002bf39704e7b2cf7f8968ae5"
);
golden!(
    gt_073_beh_shadda,
    &[0xD8, 0xA8, 0xD9, 0x91],
    &[0x02, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00],
    "037b40765af5d9590245c4d481eeecc16f98abbe44805362d393f33318f7a8d9",
    "113d9f31b7e203ba7f5d366e63258d704f8ec0c265fe07a5622445d29885b94a"
);
golden!(
    gt_074_beh_shadda_fatha,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8E],
    &[0x02, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
    "564c8f3c08e292d38da6768c7c627321ae2606f7465c6088103575d9973c9f52",
    "1c5a146e365028d20ebd4ca1f91fe3186c8386d21b3cef9c602c457e47fb441e"
);
golden!(
    gt_075_beh_shadda_damma,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8F],
    &[0x02, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00],
    "42c1e4866056f06716485cc1d6c2435c3ff5140952306ee13f4e67bfa3021885",
    "b7127d87f5adac41c5136f5e7946784ec2e216a37a0e217b74e112208f1eeae2"
);
golden!(
    gt_076_beh_shadda_kasra,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x90],
    &[0x02, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00],
    "12b43f632ef216f099dfacac2f2bf4fcc00fe0df35ba356c50761faf7e5542f6",
    "f48927063131965cfa739a15f4661fec383fc06934f4395e86a4991f64fd13b6"
);

// ═══════════════════════════════════════════════════════════════════
// GT-080 إلى GT-087: Flags (Hamza/Madda)
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_080_alef_hamza_above,
    &[0xD8, 0xA3],
    &[0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
    "6ae77f2ac3e48abba85030a92cede55e78ce0728acf47bd36df66ef2031d472e",
    "b7547fe8da2464b3f3067a1638bad055ba64a11bc3e9360fd0b6a37801f87354"
);
golden!(
    gt_081_waw_hamza_above,
    &[0xD8, 0xA4],
    &[0x1B, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
    "3e22aaeb7b98c831836c883eb194a2f91a11ea3e7ba00c42f474f9b0dce931d5",
    "4d2562d75445b3e8ba27f543f53b97b97bc86c75454edb71c113c44824a2a116"
);
golden!(
    gt_082_yeh_hamza_above,
    &[0xD8, 0xA6],
    &[0x1C, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
    "463009337640b3ffd2df0ce03bd5ad8d1d89b9869da70895d8f2927a69bafc70",
    "4a3c86269dca0f8e350a95d575b2e907eb44d9b59c512d3f7c97aedfaa017cf4"
);
golden!(
    gt_083_alef_hamza_below,
    &[0xD8, 0xA5],
    &[0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00],
    "d7f1f24cbca0fd837734f5974b2cb7bfc0a831b21da672b7bd422725c37aff03",
    "bab7d3d1d265c65ea99d63e3018879f0c1e4894943eb020021d8feda5ec5782b"
);
golden!(
    gt_084_alef_madda,
    &[0xD8, 0xA2],
    &[0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00],
    "184a6f37b8844558d161499651f40097a5a3d8a5f45644d3ca885b51c60ebb4b",
    "d44d852e33d39894a32f4a343edf667ba37e781da2fc6ce7a57f1fd78b2361aa"
);
golden!(
    gt_085_alef_hamza_above_fatha,
    &[0xD8, 0xA3, 0xD9, 0x8E],
    &[0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00],
    "8d542403ad24d72bbbbdede74fcf34ce3130c5f92b4238548bc4158da86f0510",
    "3728e311c6fe1089cfd16c0a54ffce4e9711f6c06ba98bd9e549a66f19138a0c"
);
golden!(
    gt_086_alef_hamza_below_kasra,
    &[0xD8, 0xA5, 0xD9, 0x90],
    &[0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00],
    "6cb01441429e872899444716921b732bd4300dd7acfc6a71f1e6c4b1da4bf77c",
    "239f97b0398e61075090616bd04b9a1bbb37318fcbad43989b490411c481dbdd"
);
golden!(
    gt_087_alef_madda_no_marks,
    &[0xD8, 0xA2],
    &[0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00],
    "184a6f37b8844558d161499651f40097a5a3d8a5f45644d3ca885b51c60ebb4b",
    "d44d852e33d39894a32f4a343edf667ba37e781da2fc6ce7a57f1fd78b2361aa"
);

// ═══════════════════════════════════════════════════════════════════
// GT-088 إلى GT-097: Prosody (Tanween + Superscript Alef)
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_088_noon_tanween_fath,
    &[0xD9, 0x86, 0xD9, 0x8B],
    &[0x19, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00],
    "4163ae2243aed7a756f03504fc966e69677299245331d6381be8c45b60511019",
    "e6f46a21a00fb4966ec0084a75b6c81590c3d44486ab54a84fcde7b9e74c192f"
);
golden!(
    gt_089_noon_tanween_damm,
    &[0xD9, 0x86, 0xD9, 0x8C],
    &[0x19, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00],
    "4163ae2243aed7a756f03504fc966e69677299245331d6381be8c45b60511019",
    "39a03d845441245090a8f468f0da02a5fb71a361e28102b3ceb2f294e41ab1ab"
);
golden!(
    gt_090_noon_tanween_kasr,
    &[0xD9, 0x86, 0xD9, 0x8D],
    &[0x19, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00],
    "4163ae2243aed7a756f03504fc966e69677299245331d6381be8c45b60511019",
    "ce8c6fbd0d43b8bca4fbcb900d22cd423e33cc03f387d26ef9c4a0d368879f8e"
);
golden!(
    gt_091_beh_shadda_tanween_damm,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8C],
    &[0x02, 0x00, 0x10, 0x00, 0x00, 0x02, 0x00, 0x00],
    "037b40765af5d9590245c4d481eeecc16f98abbe44805362d393f33318f7a8d9",
    "5881b86dba974fb16851db6473ad011008e4512b67d39d7d8cffa63779dd7bbc"
);
golden!(
    gt_096_waw_superscript_alef,
    &[0xD9, 0x88, 0xD9, 0xB0],
    &[0x1B, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00],
    "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a",
    "0b11617a57e4c6ad3eefb88d294442339dc5b4aaab25fc2cb061092a770af912"
);
golden!(
    gt_097_alef_superscript_alef,
    &[0xD8, 0xA7, 0xD9, 0xB0],
    &[0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00],
    "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
    "fa2325a6ddbe50436011899fec0805daa25d2ebf91903662d1d48d7a47ba0fb1"
);

// ═══════════════════════════════════════════════════════════════════
// GT-098 إلى GT-102: تسلسلات متعددة الأحرف
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_098_bismi,
    &[0xD8, 0xA8, 0xD9, 0x90, 0xD8, 0xB3, 0xD9, 0x92, 0xD9, 0x85, 0xD9, 0x90],
    &[
        0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x18, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
    ],
    "358f2811e2bc73e4c8456140e3e0e382650656f941bc6fceb69fae0c0e0ff1bd",
    "56dd2236e94a385f925b49085953aacc7c5e8455c275dd536353463462cf29e3"
);
// CR-NOTE: Spec GT-S02 stream shows LAM+SUKUN but input has no U+0652.
// Correct stream uses LAM bare. Hashes recomputed from correct stream.
golden!(
    gt_099_allah,
    &[0xD9, 0xB1, 0xD9, 0x84, 0xD9, 0x84, 0xD9, 0x91, 0xD9, 0x8E, 0xD9, 0x87],
    &[
        0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x17, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    "402b6c8b13295c3eb313892366f81c2be02e12d9172b9eafb625757de3cf57f0",
    "45b9f356ff4f0cb173bf10c612bc1a4796ae3b9bf0bc541cb856819e52cfb6a2"
);
golden!(
    gt_101_al_quran,
    &[
        0xD8, 0xA7, 0xD9, 0x84, 0xD9, 0x92, 0xD9, 0x82, 0xD9, 0x8F, 0xD8, 0xB1, 0xD9, 0x92, 0xD8,
        0xA2, 0xD9, 0x86, 0xD9, 0x8F
    ],
    &[
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x15, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x19, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ],
    "b9ef80cd36fdf5ee03806192aa9b78b2454b36c8fb28a0a2517134679eb0a6f6",
    "b053d80654a3d87e0c11b16a607ce632d46d2aa9c2c6f588eb914bc9d2319e22"
);

// ═══════════════════════════════════════════════════════════════════
// GT-103 إلى GT-116: Lam-Alef + Positional Forms
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_103_lam_alef_isolated,
    &[0xEF, 0xBB, 0xBB],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "eb458c1d9f569a53358298b4412db0949d86c20294e4bf8fa97a3fbcda41b55f",
    "d5b1ac75ce324a3716d36d15a675a6d709375a701c7998d566e3f30b006c3fca"
);
golden!(
    gt_104_lam_alef_final,
    &[0xEF, 0xBB, 0xBC],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "eb458c1d9f569a53358298b4412db0949d86c20294e4bf8fa97a3fbcda41b55f",
    "d5b1ac75ce324a3716d36d15a675a6d709375a701c7998d566e3f30b006c3fca"
);
golden!(
    gt_105_lam_alef_madda_isolated,
    &[0xEF, 0xBB, 0xB5],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00
    ],
    "bd8779e9bea8fd5d0b0b06c17127ed97cfc8f676863cc508620243cdaa50813d",
    "51d512845188314a9199aaebaaf3029118e145c50afcd8078bcf914d6e1a9932"
);
golden!(
    gt_106_lam_alef_madda_final,
    &[0xEF, 0xBB, 0xB6],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00
    ],
    "bd8779e9bea8fd5d0b0b06c17127ed97cfc8f676863cc508620243cdaa50813d",
    "51d512845188314a9199aaebaaf3029118e145c50afcd8078bcf914d6e1a9932"
);
golden!(
    gt_107_lam_alef_hamza_above_isolated,
    &[0xEF, 0xBB, 0xB7],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00
    ],
    "907d5326720a7e7d32cea0eb22ec05b4c887b68eafbbf96a854aab8992de87aa",
    "baf1a5eaf63948d4b11f813967013421aa135e89572cc19264f2404321914c00"
);
golden!(
    gt_108_lam_alef_hamza_above_final,
    &[0xEF, 0xBB, 0xB8],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00
    ],
    "907d5326720a7e7d32cea0eb22ec05b4c887b68eafbbf96a854aab8992de87aa",
    "baf1a5eaf63948d4b11f813967013421aa135e89572cc19264f2404321914c00"
);
golden!(
    gt_109_lam_alef_hamza_below_isolated,
    &[0xEF, 0xBB, 0xB9],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00
    ],
    "6477d5c298da787e421eaa83f2442ed5f271dd1bf035fc5a0d5303c2d5c11c74",
    "dfb3bd80f5b1f0b742a250fa8fa4d72948e682945923ede8fa24d9ac23f72b53"
);
golden!(
    gt_110_lam_alef_hamza_below_final,
    &[0xEF, 0xBB, 0xBA],
    &[
        0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00
    ],
    "6477d5c298da787e421eaa83f2442ed5f271dd1bf035fc5a0d5303c2d5c11c74",
    "dfb3bd80f5b1f0b742a250fa8fa4d72948e682945923ede8fa24d9ac23f72b53"
);
golden!(
    gt_111_beh_isolated,
    &[0xEF, 0xBA, 0x8F],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_112_beh_final,
    &[0xEF, 0xBA, 0x90],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_113_beh_initial,
    &[0xEF, 0xBA, 0x91],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_114_beh_medial,
    &[0xEF, 0xBA, 0x92],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_115_yeh_isolated,
    &[0xEF, 0xBB, 0xB1],
    &[0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "8aac943540c928674e5b2e38ef9f89c08b80117c051921f7cd83ef933d5d62f8",
    "4102f81b644d8e0a1b33d0fcb00addafdec8e034e45b6ba2f2b85055afbf1475"
);
golden!(
    gt_116_alef_maqsura_isolated,
    &[0xEF, 0xBB, 0xAF],
    &[0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "cbd7e522708e96087330790a3e2beb03115e545e8788400853c4056450c411cb",
    "5c5bd83ca56a56bb940b577afc28d94f1d8c00e7332888cbfe8497f1825d69c9"
);

// ═══════════════════════════════════════════════════════════════════
// GT-118 إلى GT-126: Noise Filtering + Mark Order Independence
// ═══════════════════════════════════════════════════════════════════
golden!(
    gt_118_single_space,
    &[0x20],
    &[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "ca97a20db16248f8b6568c5ee06ce360103e625b6d78041aa0ede08291500b8a",
    "9b45dad7ecb1006c7d340b506b471be0676db167eb39645d2d15db453d7bd736"
);
golden!(
    gt_119_noise_only_empty,
    &[0xD9, 0x80, 0xE2, 0x80, 0x8D, 0xEF, 0xBB, 0xBF],
    b"",
    "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0",
    "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7"
);
golden!(
    gt_120_bom_then_beh,
    &[0xEF, 0xBB, 0xBF, 0xD8, 0xA8],
    &[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
    "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3"
);
golden!(
    gt_121_zwj_between_letters,
    &[0xD8, 0xA8, 0xE2, 0x80, 0x8D, 0xD8, 0xA8],
    &[
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "2bc5f004b12841e940801c597523f5219e9445b2f775bca8cb3f9bf1b5976469",
    "25cd4d351e4aa48997e849365b94350eb9f824ba4484e46c5a668430d205a6d6"
);
golden!(
    gt_122_kashida_filtered,
    &[0xD8, 0xA8, 0xD9, 0x80, 0xD8, 0xA8],
    &[
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "2bc5f004b12841e940801c597523f5219e9445b2f775bca8cb3f9bf1b5976469",
    "25cd4d351e4aa48997e849365b94350eb9f824ba4484e46c5a668430d205a6d6"
);
golden!(
    gt_123_variation_selector_filtered,
    &[0xD8, 0xA8, 0xEF, 0xB8, 0x80, 0xD8, 0xA8],
    &[
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "2bc5f004b12841e940801c597523f5219e9445b2f775bca8cb3f9bf1b5976469",
    "25cd4d351e4aa48997e849365b94350eb9f824ba4484e46c5a668430d205a6d6"
);
golden!(
    gt_124_rtl_mark_filtered,
    &[0xD8, 0xA8, 0xE2, 0x80, 0x8F, 0xD8, 0xA8],
    &[
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ],
    "2bc5f004b12841e940801c597523f5219e9445b2f775bca8cb3f9bf1b5976469",
    "25cd4d351e4aa48997e849365b94350eb9f824ba4484e46c5a668430d205a6d6"
);

// GT-125 و GT-126: Mark Order Independence (A6)
// كلاهما يجب أن ينتج نفس النتيجة تماماً
golden!(
    gt_125_mark_order_a,
    &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x91], // BEH + FATHA + SHADDA
    &[0x02, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
    "564c8f3c08e292d38da6768c7c627321ae2606f7465c6088103575d9973c9f52",
    "1c5a146e365028d20ebd4ca1f91fe3186c8386d21b3cef9c602c457e47fb441e"
);
golden!(
    gt_126_mark_order_b,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8E], // BEH + SHADDA + FATHA
    &[0x02, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00],
    "564c8f3c08e292d38da6768c7c627321ae2606f7465c6088103575d9973c9f52",
    "1c5a146e365028d20ebd4ca1f91fe3186c8386d21b3cef9c602c457e47fb441e"
);

// ═══════════════════════════════════════════════════════════════════
// اختبارات تقاطعية صريحة (Cross-cutting property tests)
// تُثبت الخصائص A3, A6, A7 مباشرةً
// ═══════════════════════════════════════════════════════════════════

/// P6: CoreHash/PhoneticHash Separation — NOON bare vs NOON+tanween
#[test]
fn cross_p6_tanween_only_affects_phonetic() {
    let bare = process_mode_a(&[0xD9, 0x86]).unwrap();
    let tw_f = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8B]).unwrap();
    let tw_d = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8C]).unwrap();
    let tw_k = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8D]).unwrap();
    // CoreHash يجب أن يكون متطابقاً لجميع التنوينات
    assert_eq!(
        bare.core_hash, tw_f.core_hash,
        "TW_FATH must not affect CoreHash"
    );
    assert_eq!(
        bare.core_hash, tw_d.core_hash,
        "TW_DAMM must not affect CoreHash"
    );
    assert_eq!(
        bare.core_hash, tw_k.core_hash,
        "TW_KASR must not affect CoreHash"
    );
    // PhoneticHash يجب أن يختلف
    assert_ne!(bare.phonetic_hash, tw_f.phonetic_hash);
    assert_ne!(bare.phonetic_hash, tw_d.phonetic_hash);
    assert_ne!(bare.phonetic_hash, tw_k.phonetic_hash);
    // التنوينات الثلاثة تختلف عن بعضها
    assert_ne!(tw_f.phonetic_hash, tw_d.phonetic_hash);
    assert_ne!(tw_f.phonetic_hash, tw_k.phonetic_hash);
    assert_ne!(tw_d.phonetic_hash, tw_k.phonetic_hash);
}

/// A7: Digit Source Independence لكل الأرقام 0-9
#[test]
fn cross_a7_digit_source_independence_all() {
    for d in 0u8..=9u8 {
        let ascii_input = &[b'0' + d];
        let ar_cp = 0x0660u32 + d as u32;
        let ext_cp = 0x06F0u32 + d as u32;
        let ar_input: Vec<u8> = char::from_u32(ar_cp).unwrap().to_string().into_bytes();
        let ext_input: Vec<u8> = char::from_u32(ext_cp).unwrap().to_string().into_bytes();
        let r_ascii = process_mode_a(ascii_input).unwrap();
        let r_ar = process_mode_a(&ar_input).unwrap();
        let r_ext = process_mode_a(&ext_input).unwrap();
        assert_eq!(
            r_ascii.stream.to_bytes(),
            r_ar.stream.to_bytes(),
            "digit {} arabic-indic mismatch",
            d
        );
        assert_eq!(
            r_ascii.stream.to_bytes(),
            r_ext.stream.to_bytes(),
            "digit {} extended mismatch",
            d
        );
        assert_eq!(
            r_ascii.core_hash, r_ar.core_hash,
            "digit {} arabic-indic core_hash mismatch",
            d
        );
        assert_eq!(
            r_ascii.core_hash, r_ext.core_hash,
            "digit {} extended core_hash mismatch",
            d
        );
    }
}

/// A6: Mark Order Independence لكل التركيبات الصالحة
#[test]
fn cross_a6_mark_order_all_combos() {
    // BEH كقاعدة + كل التركيبات الصالحة من §2.2
    let combos: &[(&[u8], &[u8])] = &[
        // SHADDA + FATHA vs FATHA + SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8E],
            &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x91],
        ),
        // SHADDA + DAMMA vs DAMMA + SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8F],
            &[0xD8, 0xA8, 0xD9, 0x8F, 0xD9, 0x91],
        ),
        // SHADDA + KASRA vs KASRA + SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x90],
            &[0xD8, 0xA8, 0xD9, 0x90, 0xD9, 0x91],
        ),
    ];
    for (order_a, order_b) in combos {
        let ra = process_mode_a(order_a).unwrap();
        let rb = process_mode_a(order_b).unwrap();
        assert_eq!(
            ra.stream.to_bytes(),
            rb.stream.to_bytes(),
            "A6 violation: mark order affects stream"
        );
        assert_eq!(
            ra.core_hash, rb.core_hash,
            "A6 violation: mark order affects CoreHash"
        );
    }
}

/// A5: Glyph Independence — كل الأشكال الموضعية لـ BEH
#[test]
fn cross_a5_positional_forms_beh() {
    let canonical = process_mode_a(&[0xD8, 0xA8]).unwrap(); // U+0628
    let isolated = process_mode_a(&[0xEF, 0xBA, 0x8F]).unwrap(); // U+FE8F
    let final_ = process_mode_a(&[0xEF, 0xBA, 0x90]).unwrap(); // U+FE90
    let initial = process_mode_a(&[0xEF, 0xBA, 0x91]).unwrap(); // U+FE91
    let medial = process_mode_a(&[0xEF, 0xBA, 0x92]).unwrap(); // U+FE92
    for (form, result) in [
        ("isolated", &isolated),
        ("final", &final_),
        ("initial", &initial),
        ("medial", &medial),
    ] {
        assert_eq!(
            canonical.stream.to_bytes(),
            result.stream.to_bytes(),
            "A5 violation: {} form differs from canonical",
            form
        );
        assert_eq!(
            canonical.core_hash, result.core_hash,
            "A5 violation: {} form CoreHash differs",
            form
        );
    }
}

/// Lam-Alef: isolated == final لكل الأزواج الأربعة
#[test]
fn cross_lam_alef_isolated_equals_final_all_pairs() {
    let pairs: &[([u8; 3], [u8; 3])] = &[
        ([0xEF, 0xBB, 0xBB], [0xEF, 0xBB, 0xBC]), // LAM+ALEF
        ([0xEF, 0xBB, 0xB5], [0xEF, 0xBB, 0xB6]), // LAM+ALEF+MADDA
        ([0xEF, 0xBB, 0xB7], [0xEF, 0xBB, 0xB8]), // LAM+ALEF+H_ABOVE
        ([0xEF, 0xBB, 0xB9], [0xEF, 0xBB, 0xBA]), // LAM+ALEF+H_BELOW
    ];
    for (iso_bytes, fin_bytes) in pairs {
        let iso = process_mode_a(iso_bytes).unwrap();
        let fin = process_mode_a(fin_bytes).unwrap();
        assert_eq!(
            iso.stream.to_bytes(),
            fin.stream.to_bytes(),
            "Lam-Alef pair mismatch: isolated != final"
        );
        assert_eq!(iso.core_hash, fin.core_hash);
        assert_eq!(iso.phonetic_hash, fin.phonetic_hash);
    }
}
