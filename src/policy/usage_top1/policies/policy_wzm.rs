use super::super::top1_macro_init;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 0] = [];
const ONLY7: [&[u8]; 1] = [b"WZM_Main"];
const MIDDLE: [&[u8]; 1] = [b"Worker"];
const BACKEND: [&[u8]; 0] = [];

top1_macro_init!(b"Thread-", Only6);
