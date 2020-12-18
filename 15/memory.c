#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

static const uint32_t MAX_TURNS = 30000000;

typedef struct game_s {
    uint32_t *mem, t, tp;
} game_t;


static inline char*
slurp(const char *path)
{
    FILE *f = fopen(path, "r");
    assert(f);

    fseek(f, 0, SEEK_END);
    size_t sz = ftell(f);
    rewind(f);

    char *s = malloc(sz+1);
    size_t n = fread(s, 1, sz, f);
    assert(n == sz);
    s[n] = 0;

    fclose(f);
    return s;
}


static inline uint32_t
count(const char *str)
{
    uint32_t n = 0;
    for(const char *s = str; s; s = strchr(s+1, ',')) n++;
    return n;
}


static inline uint32_t*
parse(char *str, uint32_t n)
{
    uint32_t *data = malloc(n * sizeof(*data));
    uint32_t i = 0;
    for(const char *s = strtok(str, ","); s; s = strtok(NULL, ","), i++)
        data[i] = strtol(s, NULL, 10);

    return data;
}


static inline game_t
game_from_seed(uint32_t *seed, uint32_t nseed, uint32_t maxturns)
{
    uint32_t *mem = malloc(maxturns * sizeof(*mem));
    uint32_t tp = 0, t = 0;
    memset(mem, 0, maxturns * sizeof(*mem));

    while(t < nseed) {
        uint32_t n = seed[t];
        tp = mem[n];
        t++;
        if(!tp) tp = t;
        mem[n] = t;
    }

    return (game_t){ mem, tp, t };
}

static inline uint32_t
game_play_until(game_t *game, uint32_t turn)
{
    uint32_t *mem = game->mem;
    uint32_t t = game->t, tp = game->tp;
    uint32_t n = 0;
    while(__builtin_expect(t < turn, 1)) {
        n = t++ - tp;
        tp = mem[n];
        if(!tp) tp = t;
        mem[n] = t;
    }

    game->t = t;
    game->tp = tp;
    return n;
}


int main(int argc, char *argv[])
{
    assert(argc > 1);

    char *s = slurp(argv[1]);

    uint32_t n = count(s);
    uint32_t *seed = parse(s, n);
    free(s);

    game_t game = game_from_seed(seed, n, MAX_TURNS);
    free(seed);

    printf("part[1]: %u\n", game_play_until(&game, 2020));
    printf("part[1]: %u\n", game_play_until(&game, MAX_TURNS));

    free(game.mem);
    return 0;
}
