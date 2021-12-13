// weird formatting to allow vscode to collapse the
// LITERAL GODDAMN PNG IN MY CODE
const SHIP_PNG: [u8; 4096] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 220, 255, 255, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 138, 255, 255, 0, 220, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 153, 0, 20, 255, 153, 0, 75, 255, 222, 0, 230, 255,
    225, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 52, 255, 255, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 225, 0, 138, 255, 222, 0, 230, 255, 153, 0, 75, 255, 153, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 152, 0, 69, 255, 153, 0, 255, 255, 153, 0, 255, 255, 153, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 0, 179, 255, 255, 0, 179, 0, 0, 0, 0, 0, 0, 0, 0, 255, 153, 0, 138, 255, 153, 0, 255,
    255, 153, 0, 255, 255, 152, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 255, 255, 0, 73, 255, 255, 0, 166, 0, 0, 0, 0, 0, 0, 0, 0, 255, 85, 33, 69, 255,
    84, 34, 255, 255, 84, 34, 255, 255, 85, 33, 138, 255, 255, 0, 19, 255, 255, 0, 170, 255, 255,
    0, 229, 255, 255, 0, 229, 255, 255, 0, 170, 255, 255, 0, 19, 255, 85, 33, 138, 255, 84, 34,
    255, 255, 84, 34, 255, 255, 85, 33, 69, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 166, 255, 255, 0,
    73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 110, 255, 255, 0, 248, 0, 0, 0, 0, 0, 0, 0, 0, 255,
    48, 48, 69, 255, 51, 51, 255, 255, 51, 51, 255, 255, 50, 50, 138, 255, 255, 0, 28, 255, 255, 0,
    255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 28, 255, 50, 50, 138,
    255, 51, 51, 255, 255, 51, 51, 255, 255, 48, 48, 69, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 248,
    255, 255, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 155, 0, 110, 255, 157, 0, 248, 0, 0, 0, 0, 0, 0,
    0, 167, 2, 0, 0, 248, 9, 1, 1, 255, 9, 1, 1, 255, 5, 1, 1, 250, 29, 18, 0, 246, 255, 156, 0,
    255, 255, 156, 0, 255, 255, 156, 0, 255, 255, 156, 0, 255, 29, 18, 0, 246, 5, 1, 1, 250, 9, 1,
    1, 255, 9, 1, 1, 255, 2, 0, 0, 248, 0, 0, 0, 167, 0, 0, 0, 0, 255, 156, 0, 248, 255, 155, 0,
    110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 125, 12, 110, 255, 126, 12, 248, 0, 0, 0, 0, 0, 0, 0, 173,
    10, 10, 10, 255, 39, 39, 39, 255, 39, 39, 39, 255, 21, 21, 21, 255, 28, 14, 1, 255, 255, 126,
    13, 255, 255, 126, 13, 255, 255, 126, 13, 255, 255, 126, 13, 255, 28, 14, 1, 255, 21, 21, 21,
    255, 39, 39, 39, 255, 39, 39, 39, 255, 10, 10, 10, 255, 0, 0, 0, 173, 0, 0, 0, 0, 255, 126, 12,
    248, 255, 125, 12, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 51, 51, 110, 255, 50, 50, 248, 0, 0, 0, 0,
    0, 0, 0, 173, 41, 41, 41, 255, 153, 153, 153, 255, 153, 153, 153, 255, 83, 83, 83, 255, 28, 5,
    5, 255, 255, 51, 51, 255, 255, 51, 51, 255, 255, 51, 51, 255, 255, 51, 51, 255, 28, 5, 5, 255,
    83, 83, 83, 255, 153, 153, 153, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 173, 0, 0,
    0, 0, 255, 50, 50, 248, 255, 51, 51, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 66, 12, 12, 190, 111, 22,
    22, 252, 0, 0, 0, 141, 0, 0, 0, 218, 41, 41, 41, 255, 153, 153, 153, 255, 79, 201, 177, 255,
    75, 152, 137, 255, 88, 77, 77, 255, 113, 22, 22, 255, 113, 22, 22, 255, 113, 22, 22, 255, 113,
    22, 22, 255, 88, 77, 77, 255, 75, 152, 137, 255, 79, 201, 177, 255, 153, 153, 153, 255, 41, 41,
    41, 255, 0, 0, 0, 218, 0, 0, 0, 141, 111, 22, 22, 252, 66, 12, 12, 190, 0, 0, 0, 118, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    214, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41, 41, 41, 255, 153, 153, 153,
    255, 20, 241, 197, 255, 69, 208, 180, 255, 135, 135, 135, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
    0, 0, 255, 0, 0, 0, 255, 135, 135, 135, 255, 69, 208, 180, 255, 20, 241, 197, 255, 153, 153,
    153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
    214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 236, 0, 0, 0, 236, 0, 0, 0, 236,
    0, 0, 0, 236, 0, 0, 0, 252, 61, 61, 61, 255, 138, 138, 138, 255, 0, 0, 0, 255, 0, 0, 0, 255,
    41, 41, 41, 255, 153, 153, 153, 255, 20, 241, 197, 255, 5, 251, 202, 255, 25, 235, 193, 255,
    141, 141, 141, 255, 141, 141, 141, 255, 141, 141, 141, 255, 141, 141, 141, 255, 25, 235, 193,
    255, 5, 251, 202, 255, 20, 241, 197, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0,
    0, 0, 255, 138, 138, 138, 255, 61, 61, 61, 255, 0, 0, 0, 252, 0, 0, 0, 236, 0, 0, 0, 236, 0, 0,
    0, 236, 0, 0, 0, 236, 0, 0, 0, 255, 20, 20, 20, 255, 34, 34, 34, 255, 34, 34, 34, 255, 5, 5, 5,
    255, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41, 41, 41, 255, 153,
    153, 153, 255, 50, 221, 187, 255, 18, 242, 197, 255, 17, 243, 198, 255, 153, 153, 153, 255,
    153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 243, 198, 255, 18, 242, 197,
    255, 50, 221, 187, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0, 0, 0, 255, 149,
    149, 149, 255, 66, 66, 66, 255, 5, 5, 5, 255, 34, 34, 34, 255, 34, 34, 34, 255, 20, 20, 20,
    255, 0, 0, 0, 255, 0, 0, 0, 255, 90, 90, 90, 255, 153, 153, 153, 255, 153, 153, 153, 255, 24,
    24, 24, 255, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41, 41, 41, 255,
    153, 153, 153, 255, 153, 153, 153, 255, 83, 199, 176, 255, 17, 243, 198, 255, 153, 153, 153,
    255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 243, 198, 255, 83, 199,
    176, 255, 153, 153, 153, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0, 0, 0, 255,
    149, 149, 149, 255, 66, 66, 66, 255, 24, 24, 24, 255, 153, 153, 153, 255, 153, 153, 153, 255,
    90, 90, 90, 255, 0, 0, 0, 255, 0, 0, 0, 255, 90, 90, 90, 255, 153, 153, 153, 255, 153, 153,
    153, 255, 24, 24, 24, 255, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41,
    41, 41, 255, 153, 153, 153, 255, 84, 198, 175, 255, 76, 204, 178, 255, 87, 196, 174, 255, 153,
    153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 87, 196, 174, 255,
    76, 204, 178, 255, 84, 198, 175, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0, 0,
    0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 24, 24, 24, 255, 153, 153, 153, 255, 153, 153,
    153, 255, 90, 90, 90, 255, 0, 0, 0, 255, 0, 0, 0, 255, 90, 90, 90, 255, 153, 153, 153, 255,
    153, 153, 153, 255, 24, 24, 24, 255, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0,
    0, 255, 41, 41, 41, 255, 153, 153, 153, 255, 20, 241, 197, 255, 69, 208, 180, 255, 153, 153,
    153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153,
    153, 153, 255, 69, 208, 180, 255, 20, 241, 197, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0,
    0, 255, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 24, 24, 24, 255, 153, 153, 153, 255,
    153, 153, 153, 255, 90, 90, 90, 255, 0, 0, 0, 255, 0, 0, 0, 255, 16, 16, 16, 255, 28, 28, 28,
    255, 28, 28, 28, 255, 4, 4, 4, 255, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0,
    255, 41, 41, 41, 255, 153, 153, 153, 255, 128, 169, 161, 255, 80, 106, 101, 255, 42, 42, 42,
    255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 42, 42,
    42, 255, 80, 106, 101, 255, 128, 169, 161, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0,
    255, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 4, 4, 4, 255, 28, 28, 28, 255, 28, 28,
    28, 255, 16, 16, 16, 255, 0, 0, 0, 255, 0, 0, 0, 207, 0, 0, 0, 235, 0, 0, 0, 243, 0, 0, 0, 207,
    0, 0, 0, 247, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41, 41, 41, 255,
    153, 153, 153, 255, 128, 128, 128, 255, 67, 67, 67, 255, 17, 17, 17, 255, 153, 153, 153, 255,
    153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 17, 17, 255, 67, 67, 67, 255,
    128, 128, 128, 255, 153, 153, 153, 255, 41, 41, 41, 255, 0, 0, 0, 255, 0, 0, 0, 255, 149, 149,
    149, 255, 66, 66, 66, 255, 0, 0, 0, 247, 0, 0, 0, 207, 0, 0, 0, 243, 0, 0, 0, 235, 0, 0, 0,
    207, 0, 0, 0, 0, 0, 0, 0, 150, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0, 0, 214, 66, 66, 66, 255, 149,
    149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 255, 41, 41, 41, 255, 153, 153, 153, 255, 20, 20, 20,
    255, 0, 0, 0, 255, 17, 17, 17, 255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255,
    153, 153, 153, 255, 17, 17, 17, 255, 0, 0, 0, 255, 20, 20, 20, 255, 153, 153, 153, 255, 41, 41,
    41, 255, 0, 0, 0, 255, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 0, 0, 0, 214, 0, 0,
    0, 0, 0, 0, 0, 191, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 99, 0, 0, 0, 0,
    0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 171, 32, 32, 32, 165,
    79, 79, 79, 255, 17, 17, 17, 148, 0, 0, 0, 188, 17, 17, 17, 255, 153, 153, 153, 255, 153, 153,
    153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 17, 17, 255, 0, 0, 0, 188, 17, 17, 17,
    148, 79, 79, 79, 255, 32, 32, 32, 165, 0, 0, 0, 171, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66,
    66, 255, 0, 0, 0, 214, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0,
    0, 0, 81, 0, 0, 0, 69, 0, 0, 0, 255, 0, 0, 0, 34, 0, 0, 0, 116, 17, 17, 17, 255, 153, 153, 153,
    255, 153, 153, 153, 255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 17, 17, 255, 0, 0, 0, 116,
    0, 0, 0, 34, 0, 0, 0, 255, 0, 0, 0, 69, 0, 0, 0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66,
    66, 255, 0, 0, 0, 214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0,
    0, 81, 0, 0, 0, 69, 0, 0, 0, 255, 0, 0, 0, 34, 0, 0, 0, 116, 3, 21, 25, 255, 34, 192, 232, 255,
    34, 192, 232, 255, 34, 192, 232, 255, 34, 192, 232, 255, 3, 21, 26, 255, 0, 0, 0, 116, 0, 0, 0,
    34, 0, 0, 0, 255, 0, 0, 0, 69, 0, 0, 0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255,
    0, 0, 0, 214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 81, 0,
    0, 0, 64, 0, 0, 0, 236, 0, 0, 0, 32, 0, 0, 0, 116, 1, 22, 27, 255, 11, 200, 247, 255, 11, 200,
    247, 255, 11, 200, 247, 255, 11, 200, 247, 255, 1, 22, 27, 255, 0, 0, 0, 116, 0, 0, 0, 32, 0,
    0, 0, 236, 0, 0, 0, 64, 0, 0, 0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 0, 0,
    0, 214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 81, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 17, 17, 17, 255, 153, 153, 153, 255, 153, 153, 153,
    255, 153, 153, 153, 255, 153, 153, 153, 255, 17, 17, 17, 255, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 0, 0, 0, 214,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 214, 66, 66, 66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 15, 15, 15, 154, 84, 84, 84, 255, 84, 84, 84, 255, 84, 84, 84,
    255, 84, 84, 84, 255, 15, 15, 15, 154, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66, 66, 66, 255, 0, 0, 0, 214, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 66, 66,
    66, 255, 149, 149, 149, 255, 0, 0, 0, 255, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 28, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 255, 149, 149, 149, 255, 66,
    66, 66, 255, 0, 0, 0, 214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 29, 29, 29, 147, 39, 39, 39, 250, 0, 0, 0, 66, 0, 0, 0,
    21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 66, 0, 0, 0, 199, 0,
    0, 0, 199, 0, 0, 0, 66, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    21, 0, 0, 0, 66, 39, 39, 39, 250, 29, 29, 29, 147, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0,
    0, 0, 248, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 110, 0, 0, 0, 248, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0, 0, 110, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 179, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 0, 0,
    0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 179, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 179, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0,
    179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_render::texture;
use space_rocks_web::{
    input::{input_parser, movements_to_vel},
    map::RaceMap,
    network::get_map_from_server,
    objects::{timer_system, Ship},
    physics::{
        angular::{angular_momentum, AngularMomentum},
        velocity::{velocity, Velocity},
    },
};
use wasm_bindgen_futures::spawn_local;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen;

#[derive(StageLabel, SystemLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Stages {
    InputHandling,
    Physics,
    ScreenAdjustment,
}

fn main() {
    spawn_local(run_app())
}

async fn run_app() {
    println!("Hello, world!");

    // when building for wasm, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let g = get_map_from_server().await;

    match g {
        Ok(map) => create_app(&map),
        Err(_e) => panic!("Unable to parse json map from server!"),
    }
}

fn create_app(map: &RaceMap) {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(map.clone_me());

    app.add_startup_system(setup.system())
        .add_system(input_parser.system().label(Stages::InputHandling))
        .add_system(
            movements_to_vel
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            velocity
                .system()
                .after(Stages::InputHandling)
                .label(Stages::Physics),
        )
        .add_system(
            angular_momentum
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            follows_ship
                .system()
                .after(Stages::Physics)
                .label(Stages::ScreenAdjustment),
        )
        .add_system(timer_system.system())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    map: Option<Res<RaceMap>>,
) {
    // let bevy handle changed assets in real time
    // but only if running on native
    #[cfg(not(target_arch = "wasm32"))]
    asset_server.watch_for_changes().unwrap();

    // make it look like i put effort into this
    let primary_window = windows.get_primary_mut().unwrap();

    // don't hide cursor if we are in dev build so we can use inspector
    #[cfg(debug_assertions)]
    primary_window.set_cursor_visibility(false);

    // primary_window.set_resizable(false);
    // primary_window.set_scale_factor_override(Some(1.0));
    #[cfg(target_arch = "wasm32")]
    primary_window.set_resolution(1280.0, 720.0);
    #[cfg(not(target_arch = "wasm32"))]
    primary_window.set_resolution(200.0, 200.0);

    primary_window
        .set_title("Entirely Too Many Rocks, Just Seriously, There Are Too Many.".to_owned());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(FollowsShip);

    let ship_texture = textures.add(texture::Texture::new(
        texture::Extent3d {
            width: 32,
            height: 32,
            depth: 1,
        },
        texture::TextureDimension::D1,
        SHIP_PNG.to_vec(),
        texture::TextureFormat::Rgba8Unorm,
    ));

    // ship
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(
                ColorMaterial {
                    texture: Some(ship_texture),
                    ..Default::default()
                }
                .into(),
            ),
            ..Default::default()
        })
        .insert(Ship::default())
        .insert(Velocity(0.0, 0.0, 0.4))
        .insert(AngularMomentum(0.0, 0.8))
        .insert(ColliderBundle {
            shape: ColliderShape::round_cuboid(1.0, 1.0, 0.3),
            collider_type: ColliderType::Solid,
            material: ColliderMaterial {
                friction: 0.7,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased,
            position: Vec2::new(30.0, 30.0).into(),
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            },
            activation: RigidBodyActivation::cannot_sleep(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        });

    let handle = materials.add(ColorMaterial::color(Color::WHITE));
    if let Some(m) = map {
        m.put_into_world(&mut commands, handle, meshes);
    }
}

struct FollowsShip;

fn follows_ship(
    mut query: QuerySet<(
        Query<&mut Transform, With<FollowsShip>>,
        Query<&Transform, With<Ship>>,
    )>,
) {
    let ship_pos = query.q1().single().unwrap().translation;

    for mut trans in query.q0_mut().iter_mut() {
        trans.translation = Vec3::new(
            ship_pos.x,
            ship_pos.y,
            trans.translation.z, // maintain previous visibility
        );
    }
}
