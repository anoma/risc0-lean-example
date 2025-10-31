#include <stdint.h>
#include <stddef.h>
#include <sys/time.h>

int _gettimeofday (struct timeval *__p, void *__tz) {
  return -1;
}

unsigned int __atomic_fetch_sub_4(volatile void *ptr, unsigned int val, int memorder)
{
    uint32_t* p = (uint32_t*)ptr;
    unsigned int old = *p;
    *p = old - val;
    return old;
}

#define SBRK_MAX_HEAP 64*1024*1024
static unsigned char sbrk_heap[SBRK_MAX_HEAP];
static ptrdiff_t sbrk_bkrp = 0;

void* _sbrk (ptrdiff_t __incr) {
    ptrdiff_t free = sbrk_bkrp;
    sbrk_bkrp += __incr;
    if(sbrk_bkrp > SBRK_MAX_HEAP) {
        return NULL;
    }
    return &sbrk_heap[free];
}

void _Unwind_Resume() {}
void _Unwind_RaiseException() {}
void _Unwind_Resume_or_Rethrow() {}
void _Unwind_GetTextRelBase() {}
void _Unwind_GetDataRelBase() {}
void _Unwind_DeleteException() {}
void _Unwind_GetRegionStart() {}
void _Unwind_GetLanguageSpecificData() {}
void _Unwind_GetIPInfo() {}
void _Unwind_SetGR() {}
void _Unwind_GetGR() {}
void _Unwind_SetIP() {}

int _kill(int pid, int sig) { return -1; }
int _getpid() { return 1; }
void _exit(int status) { while(1); }
int _fstat() { return -1; }
int _isatty() { return -1; }
int _lseek() { return -1; }
int _read() { return -1; }
int _write() { return -1; }
int _open() { return -1; }
int _close() { return -1; }
