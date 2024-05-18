// SPDX-License-Identifier: GPL-2.0-only OR MIT
/*

Copyright (C) 2023 The Falco Authors.

This file is dual licensed under either the MIT or GPL 2. See MIT.txt
or GPL2.txt for full copies of the license.

*/

/*
 * This file was automatically created by syscalls-bumper (https://github.com/falcosecurity/syscalls-bumper).")
 * DO NOT EDIT THIS FILE MANUALLY.")
 */

#include "ppm_events_public.h"

/*
 * This table is used by drivers when receiving a 32bit syscall.
 * It is needed to convert a 32bit syscall (the array index) to a 64bit syscall value.
 * NOTE: some syscalls might be unavailable on x86_64; their value will be set to -1.
 * Some unavailable syscalls are identical to a compatible x86_64 syscall; in those cases,
 * we use the compatible x86_64 syscall, eg: mmap2 -> mmap.
 */
const int g_ia32_64_map[SYSCALL_TABLE_SIZE] = {
	[0] = 219,
	[1] = 60,
	[2] = 57,
	[3] = 0,
	[4] = 1,
	[5] = 2,
	[6] = 3,
	[7] = -1, // ia32 only: waitpid
	[8] = 85,
	[9] = 86,
	[10] = 87,
	[11] = 59,
	[12] = 80,
	[13] = 201,
	[14] = 133,
	[15] = 90,
	[16] = 94,
	[18] = -1, // ia32 only: oldstat
	[19] = 8,
	[20] = 39,
	[21] = 165,
	[22] = 166, // NOTE: syscall umount unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[23] = 105,
	[24] = 102,
	[25] = -1, // ia32 only: stime
	[26] = 101,
	[27] = 37,
	[28] = -1, // ia32 only: oldfstat
	[29] = 34,
	[30] = 132,
	[33] = 21,
	[34] = -1, // ia32 only: nice
	[36] = 162,
	[37] = 62,
	[38] = 82,
	[39] = 83,
	[40] = 84,
	[41] = 32,
	[42] = 22,
	[43] = 100,
	[45] = 12,
	[46] = 106,
	[47] = 104,
	[48] = -1, // ia32 only: signal
	[49] = 107,
	[50] = 108,
	[51] = 163,
	[52] = 166,
	[54] = 16,
	[55] = 72,
	[57] = 109,
	[59] = -1, // ia32 only: oldolduname
	[60] = 95,
	[61] = 161,
	[62] = 136,
	[63] = 33,
	[64] = 110,
	[65] = 111,
	[66] = 112,
	[67] = -1, // ia32 only: sigaction
	[68] = -1, // ia32 only: sgetmask
	[69] = -1, // ia32 only: ssetmask
	[70] = 113,
	[71] = 114,
	[72] = -1, // ia32 only: sigsuspend
	[73] = -1, // ia32 only: sigpending
	[74] = 170,
	[75] = 160,
	[76] = 97,
	[77] = 98,
	[78] = 96,
	[79] = 164,
	[80] = 115,
	[81] = 116,
	[82] = 23,
	[83] = 88,
	[84] = -1, // ia32 only: oldlstat
	[85] = 89,
	[87] = 167,
	[88] = 169,
	[89] = -1, // ia32 only: readdir
	[90] = 9,
	[91] = 11,
	[92] = 76,
	[93] = 77,
	[94] = 91,
	[95] = 93,
	[96] = 140,
	[97] = 141,
	[99] = 137,
	[100] = 138,
	[101] = 173,
	[102] = -1, // ia32 only: socketcall
	[103] = 103,
	[104] = 38,
	[105] = 36,
	[106] = 4,
	[107] = 6,
	[108] = 5,
	[109] = -1, // ia32 only: olduname
	[110] = 172,
	[111] = 153,
	[113] = -1, // ia32 only: vm86old
	[114] = 61,
	[115] = 168,
	[116] = 99,
	[117] = -1, // ia32 only: ipc
	[118] = 74,
	[119] = -1, // ia32 only: sigreturn
	[120] = 56,
	[121] = 171,
	[122] = 63,
	[123] = 154,
	[124] = 159,
	[125] = 10,
	[126] = -1, // ia32 only: sigprocmask
	[128] = 175,
	[129] = 176,
	[131] = 179,
	[132] = 121,
	[133] = 81,
	[135] = 139,
	[136] = 135,
	[138] = 122,
	[139] = 123,
	[140] = 8, // NOTE: syscall _llseek unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[141] = 78,
	[142] = -1, // ia32 only: _newselect
	[143] = 73,
	[144] = 26,
	[145] = 19,
	[146] = 20,
	[147] = 124,
	[148] = 75,
	[150] = 149,
	[151] = 150,
	[152] = 151,
	[153] = 152,
	[154] = 142,
	[155] = 143,
	[156] = 144,
	[157] = 145,
	[158] = 24,
	[159] = 146,
	[160] = 147,
	[161] = 148,
	[162] = 35,
	[163] = 25,
	[164] = 117,
	[165] = 118,
	[166] = -1, // ia32 only: vm86
	[168] = 7,
	[170] = 119,
	[171] = 120,
	[172] = 157,
	[173] = 15,
	[174] = 13,
	[175] = 14,
	[176] = 127,
	[177] = 128,
	[178] = 129,
	[179] = 130,
	[180] = 17,
	[181] = 18,
	[182] = 92,
	[183] = 79,
	[184] = 125,
	[185] = 126,
	[186] = 131,
	[187] = 40,
	[190] = 58,
	[191] = 97, // NOTE: syscall ugetrlimit unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[192] = 9, // NOTE: syscall mmap2 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[193] = -1, // ia32 only: truncate64
	[194] = -1, // ia32 only: ftruncate64
	[195] = 4, // NOTE: syscall stat64 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[196] = 6, // NOTE: syscall lstat64 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[197] = 5, // NOTE: syscall fstat64 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[198] = -1, // ia32 only: lchown32
	[199] = 102, // NOTE: syscall getuid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[200] = 104, // NOTE: syscall getgid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[201] = 107, // NOTE: syscall geteuid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[202] = 108, // NOTE: syscall getegid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[203] = -1, // ia32 only: setreuid32
	[204] = -1, // ia32 only: setregid32
	[205] = -1, // ia32 only: getgroups32
	[206] = -1, // ia32 only: setgroups32
	[207] = -1, // ia32 only: fchown32
	[208] = 117, // NOTE: syscall setresuid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[209] = 118, // NOTE: syscall getresuid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[210] = 119, // NOTE: syscall setresgid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[211] = 120, // NOTE: syscall getresgid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[212] = -1, // ia32 only: chown32
	[213] = 105, // NOTE: syscall setuid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[214] = 106, // NOTE: syscall setgid32 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[215] = -1, // ia32 only: setfsuid32
	[216] = -1, // ia32 only: setfsgid32
	[217] = 155,
	[218] = 27,
	[219] = 28,
	[220] = 217,
	[221] = 72, // NOTE: syscall fcntl64 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[224] = 186,
	[225] = 187,
	[226] = 188,
	[227] = 189,
	[228] = 190,
	[229] = 191,
	[230] = 192,
	[231] = 193,
	[232] = 194,
	[233] = 195,
	[234] = 196,
	[235] = 197,
	[236] = 198,
	[237] = 199,
	[238] = 200,
	[239] = 40, // NOTE: syscall sendfile64 unmapped on x86_64, forcefully mapped to compatible syscall. See syscalls-bumper bumpIA32to64Map() call.
	[240] = 202,
	[241] = 203,
	[242] = 204,
	[243] = 205,
	[244] = 211,
	[245] = 206,
	[246] = 207,
	[247] = 208,
	[248] = 209,
	[249] = 210,
	[250] = 221,
	[252] = 231,
	[253] = 212,
	[254] = 213,
	[255] = 233,
	[256] = 232,
	[257] = 216,
	[258] = 218,
	[259] = 222,
	[260] = 223,
	[261] = 224,
	[262] = 225,
	[263] = 226,
	[264] = 227,
	[265] = 228,
	[266] = 229,
	[267] = 230,
	[268] = -1, // ia32 only: statfs64
	[269] = -1, // ia32 only: fstatfs64
	[270] = 234,
	[271] = 235,
	[272] = -1, // ia32 only: fadvise64_64
	[274] = 237,
	[275] = 239,
	[276] = 238,
	[277] = 240,
	[278] = 241,
	[279] = 242,
	[280] = 243,
	[281] = 244,
	[282] = 245,
	[283] = 246,
	[284] = 247,
	[286] = 248,
	[287] = 249,
	[288] = 250,
	[289] = 251,
	[290] = 252,
	[291] = 253,
	[292] = 254,
	[293] = 255,
	[294] = 256,
	[295] = 257,
	[296] = 258,
	[297] = 259,
	[298] = 260,
	[299] = 261,
	[300] = -1, // ia32 only: fstatat64
	[301] = 263,
	[302] = 264,
	[303] = 265,
	[304] = 266,
	[305] = 267,
	[306] = 268,
	[307] = 269,
	[308] = 270,
	[309] = 271,
	[310] = 272,
	[311] = 273,
	[312] = 274,
	[313] = 275,
	[314] = 277,
	[315] = 276,
	[316] = 278,
	[317] = 279,
	[318] = 309,
	[319] = 281,
	[320] = 280,
	[321] = 282,
	[322] = 283,
	[323] = 284,
	[324] = 285,
	[325] = 286,
	[326] = 287,
	[327] = 289,
	[328] = 290,
	[329] = 291,
	[330] = 292,
	[331] = 293,
	[332] = 294,
	[333] = 295,
	[334] = 296,
	[335] = 297,
	[336] = 298,
	[337] = 299,
	[338] = 300,
	[339] = 301,
	[340] = 302,
	[341] = 303,
	[342] = 304,
	[343] = 305,
	[344] = 306,
	[345] = 307,
	[346] = 308,
	[347] = 310,
	[348] = 311,
	[349] = 312,
	[350] = 313,
	[351] = 314,
	[352] = 315,
	[353] = 316,
	[354] = 317,
	[355] = 318,
	[356] = 319,
	[357] = 321,
	[358] = 322,
	[359] = 41,
	[360] = 53,
	[361] = 49,
	[362] = 42,
	[363] = 50,
	[364] = 288,
	[365] = 55,
	[366] = 54,
	[367] = 51,
	[368] = 52,
	[369] = 44,
	[370] = 46,
	[371] = 45,
	[372] = 47,
	[373] = 48,
	[374] = 323,
	[375] = 324,
	[376] = 325,
	[377] = 326,
	[378] = 327,
	[379] = 328,
	[380] = 329,
	[381] = 330,
	[382] = 331,
	[383] = 332,
	[384] = 158,
	[385] = 333,
	[386] = 334,
	[393] = 64,
	[394] = 66,
	[395] = 29,
	[396] = 31,
	[397] = 30,
	[398] = 67,
	[399] = 68,
	[400] = 69,
	[401] = 70,
	[402] = 71,
	[403] = -1, // ia32 only: clock_gettime64
	[404] = -1, // ia32 only: clock_settime64
	[405] = -1, // ia32 only: clock_adjtime64
	[406] = -1, // ia32 only: clock_getres_time64
	[407] = -1, // ia32 only: clock_nanosleep_time64
	[408] = -1, // ia32 only: timer_gettime64
	[409] = -1, // ia32 only: timer_settime64
	[410] = -1, // ia32 only: timerfd_gettime64
	[411] = -1, // ia32 only: timerfd_settime64
	[412] = -1, // ia32 only: utimensat_time64
	[413] = -1, // ia32 only: pselect6_time64
	[414] = -1, // ia32 only: ppoll_time64
	[416] = -1, // ia32 only: io_pgetevents_time64
	[417] = -1, // ia32 only: recvmmsg_time64
	[418] = -1, // ia32 only: mq_timedsend_time64
	[419] = -1, // ia32 only: mq_timedreceive_time64
	[420] = -1, // ia32 only: semtimedop_time64
	[421] = -1, // ia32 only: rt_sigtimedwait_time64
	[422] = -1, // ia32 only: futex_time64
	[423] = -1, // ia32 only: sched_rr_get_interval_time64
	[424] = 424,
	[425] = 425,
	[426] = 426,
	[427] = 427,
	[428] = 428,
	[429] = 429,
	[430] = 430,
	[431] = 431,
	[432] = 432,
	[433] = 433,
	[434] = 434,
	[435] = 435,
	[436] = 436,
	[437] = 437,
	[438] = 438,
	[439] = 439,
	[440] = 440,
	[441] = 441,
	[442] = 442,
	[443] = 443,
	[444] = 444,
	[445] = 445,
	[446] = 446,
	[447] = 447,
	[448] = 448,
	[449] = 449,
	[450] = 450,
	[451] = 451,
	[452] = 452,
	[453] = 453,
	[454] = 454,
	[455] = 455,
	[456] = 456,
};
