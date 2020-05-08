#ifndef MMHEROES_CORE_H
#define MMHEROES_CORE_H

/* ------------------------------------------------------------------------------------ */
/*                  Warning! This file has been autogenerated by cbindgen.              */
/*                               Don't modify it manually.                              */
/* ------------------------------------------------------------------------------------ */

#include <stdint.h>
#include <stdbool.h>

#define MMHEROES_HELP_SCREEN_OPTION_COUNT 7

/**
 * Максимальное число возможных вариантов на главном экране.
 */
#define MMHEROES_MAX_OPTIONS_IN_SCENE_ROUTER 12

#define MMHEROES_NUM_DAYS 6

#define MMHEROES_NUM_SUBJECTS 6

typedef enum {
  MMHEROES_Color_Black = 0,
  MMHEROES_Color_Red = 1,
  MMHEROES_Color_Yellow = 3,
  MMHEROES_Color_Blue = 4,
  MMHEROES_Color_Magenta = 5,
  MMHEROES_Color_Cyan = 6,
  MMHEROES_Color_White = 7,
  MMHEROES_Color_Gray = 8,
  MMHEROES_Color_RedBright = 9,
  MMHEROES_Color_Green = 10,
  MMHEROES_Color_YellowBright = 11,
  MMHEROES_Color_BlueBright = 12,
  MMHEROES_Color_MagentaBright = 13,
  MMHEROES_Color_CyanBright = 14,
  MMHEROES_Color_WhiteBright = 15,
} MMHEROES_Color;

/**
 * The game mode selector.
 */
typedef enum {
  /**
   * Normal game mode, the character has average characteristics.
   * This is the default.
   */
  MMHEROES_GameMode_Normal,
  /**
   * The player will be prompted to select initial character characteristics:
   * - Random student, same as `Normal` mode.
   * - Clever student: better brain, but worse stamina and charisma
   * - Impudent student: better stamina, but worse brain and charisma
   * - Sociable student: better charisma, but worse brain and stamina.
   */
  MMHEROES_GameMode_SelectInitialParameters,
  /**
   * Same as `SelectInitialParameters`, but another option is available —
   * God mode. It enables maximum abilities.
   *
   * This mode is enable by passing a special flag to the executable.
   */
  MMHEROES_GameMode_God,
} MMHEROES_GameMode;

typedef enum {
  MMHEROES_Input_KeyUp,
  MMHEROES_Input_KeyDown,
  MMHEROES_Input_Enter,
  MMHEROES_Input_Other,
} MMHEROES_Input;

typedef struct MMHEROES_Game MMHEROES_Game;

typedef struct MMHEROES_GameUI MMHEROES_GameUI;

typedef struct MMHEROES_InputRecorder_InputRecorderSink MMHEROES_InputRecorder_InputRecorderSink;

typedef void *MMHEROES_AllocatorContext;

/**
 * Функция, принимающая в качестве первого аргумента некоторый контекст,
 * в качестве второго аргумента размер выделяемого блока памяти,
 * а в качестве третьего — выравнивание.
 */
typedef void *(*MMHEROES_Allocator)(MMHEROES_AllocatorContext, uintptr_t, uintptr_t);

/**
 * Функция, принимающая в качестве первого аргумента некоторый контекст,
 * в качестве второго — указатель на освобождаемый блок памяти,
 * а в качестве третьего — размер освобождаемого блока.
 */
typedef void (*MMHEROES_Deallocator)(MMHEROES_AllocatorContext, void*, uintptr_t);

/**
 * Количество часов, прошедших с полуночи. Имеет семантику таймстэмпа, то есть,
 * экземпляры этого типа нельзя складывать, но к ним можно прибавлять экземпляры
 * типа `Duration` и получать новый экземпляр типа `Time`.
 */
typedef uint8_t MMHEROES_Time;

typedef struct {
  void *context;
  bool (*sink)(void*, const uint8_t*, uintptr_t);
} MMHEROES_InputRecorderSink;

typedef struct {
  const uint8_t *buf;
  uintptr_t len;
} MMHEROES_RendererRequestIterator;

typedef int32_t MMHEROES_Milliseconds;

typedef enum {
  MMHEROES_RendererRequest_ClearScreen,
  MMHEROES_RendererRequest_Flush,
  MMHEROES_RendererRequest_WriteStr,
  MMHEROES_RendererRequest_MoveCursor,
  MMHEROES_RendererRequest_SetColor,
  MMHEROES_RendererRequest_Sleep,
} MMHEROES_RendererRequest_Tag;

typedef struct {
  const uint8_t *buf;
  uintptr_t length;
} MMHEROES_RendererRequest_MMHEROES_WriteStr_Body;

typedef struct {
  uint8_t line;
  uint8_t column;
} MMHEROES_RendererRequest_MMHEROES_MoveCursor_Body;

typedef struct {
  MMHEROES_Color foreground;
  MMHEROES_Color background;
} MMHEROES_RendererRequest_MMHEROES_SetColor_Body;

typedef struct {
  MMHEROES_Milliseconds milliseconds;
} MMHEROES_RendererRequest_MMHEROES_Sleep_Body;

typedef struct {
  MMHEROES_RendererRequest_Tag tag;
  union {
    MMHEROES_RendererRequest_MMHEROES_WriteStr_Body write_str;
    MMHEROES_RendererRequest_MMHEROES_MoveCursor_Body move_cursor;
    MMHEROES_RendererRequest_MMHEROES_SetColor_Body set_color;
    MMHEROES_RendererRequest_MMHEROES_Sleep_Body sleep;
  };
} MMHEROES_RendererRequest;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Продолжает игру до следующего запроса на нажатие клавиши.
 *
 * При первом вызове этой функции неважно, что передаётся в параметре `input`.
 */
bool mmheroes_continue(MMHEROES_GameUI *game_ui,
                       MMHEROES_Input input);

MMHEROES_Game *mmheroes_game_create(MMHEROES_GameMode mode,
                                    uint64_t seed,
                                    MMHEROES_AllocatorContext allocator_context,
                                    MMHEROES_Allocator allocator);

void mmheroes_game_destroy(MMHEROES_Game *game,
                           MMHEROES_AllocatorContext deallocator_context,
                           MMHEROES_Deallocator deallocator);

/**
 * Число возможных вариантов для выбора.
 *
 * Аргумент `game` не должен быть нулевым указателем, иначе UB.
 */
uintptr_t mmheroes_game_get_available_actions(MMHEROES_Game *game);

/**
 * Записывает текущий игровой день и время в аргументы `out_day` и `out_time`
 * и возвращает `true` если они доступны, иначе не трогает аргументы и возвращает
 * `false`.
 *
 * Игровой день и время могут быть недоступны, например, если игра ещё не началась.
 */
bool mmheroes_game_get_current_time(MMHEROES_Game *game,
                                    uint8_t *out_day,
                                    MMHEROES_Time *out_time);

MMHEROES_GameUI *mmheroes_game_ui_create(MMHEROES_Game *game,
                                         MMHEROES_AllocatorContext allocator_context,
                                         MMHEROES_Allocator allocator);

void mmheroes_game_ui_destroy(MMHEROES_GameUI *game_ui,
                              MMHEROES_AllocatorContext deallocator_context,
                              MMHEROES_Deallocator deallocator);

MMHEROES_InputRecorder_InputRecorderSink *mmheroes_input_recorder_create(MMHEROES_InputRecorderSink *sink,
                                                                         MMHEROES_AllocatorContext allocator_context,
                                                                         MMHEROES_Allocator allocator);

void mmheroes_input_recorder_destroy(MMHEROES_InputRecorder_InputRecorderSink *recorder,
                                     MMHEROES_AllocatorContext deallocator_context,
                                     MMHEROES_Deallocator deallocator);

bool mmheroes_input_recorder_flush(MMHEROES_InputRecorder_InputRecorderSink *recorder);

bool mmheroes_input_recorder_record(MMHEROES_InputRecorder_InputRecorderSink *recorder,
                                    MMHEROES_Input input);

/**
 * Инициализирует итератор по запросам на рендеринг.
 * `game_ui` должен быть валидный ненулевой указатель.
 */
void mmheroes_renderer_request_iterator_begin(MMHEROES_RendererRequestIterator *iterator,
                                              const MMHEROES_GameUI *game_ui);

/**
 * Продвигает итератор по запросам на рендеринг.
 *
 * Возвращает `true` и записывает в параметр `out` следующий запрос, если он есть.
 *
 * Возвращает `false`, если запросов больше нет.
 */
bool mmheroes_renderer_request_iterator_next(MMHEROES_RendererRequestIterator *iterator,
                                             MMHEROES_RendererRequest *out);

/**
 * Воспроизводит игру с помощью входных данных, записанных ранее с помощью
 * `InputRecorder`.
 *
 * В случае ошибки возвращает `false`, иначе — `true`.
 */
bool mmheroes_replay(MMHEROES_GameUI *game_ui,
                     const uint8_t *recorded_input,
                     uintptr_t recorded_input_len);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* MMHEROES_CORE_H */
