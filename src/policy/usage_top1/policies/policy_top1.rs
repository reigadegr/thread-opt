use super::super::top1_macro_init;

const TOP: [&[u8]; 1] = [b"FAsync"];
const ONLY6: [&[u8]; 1] = [b"RHIThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 1] = [b"RenderThread"];
const BACKEND: [&[u8]; 0] = [];

top1_macro_init!(b"Thread-", Only7);
