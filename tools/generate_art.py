from __future__ import annotations

import json
import math
from pathlib import Path

from PIL import Image, ImageColor, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[1]
REQ = json.loads((ROOT / "assets/data/sprite_requirements.json").read_text(encoding="utf-8"))
OUT = ROOT / "assets/generated"
JOURNAL = ["routes", "notes", "brews", "greenhouse", "rapport"]
TOASTS = ["journal_note", "recipe_logged", "quest_accepted", "quest_complete", "route_restored", "best_quality"]


def rgb(code, a=255):
    r, g, b = ImageColor.getrgb(code)
    return r, g, b, a


def box(cx, cy, rx, ry):
    return cx - rx, cy - ry, cx + rx, cy + ry


def save(img, rel):
    path = OUT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)
    print(path.relative_to(ROOT))


def sheet(colors):
    img = Image.new("RGBA", (320, 256), (0, 0, 0, 0))
    cloak, trim, hair, skin, accent = map(rgb, colors)
    for row, facing in enumerate(["down", "left", "right", "up"]):
        for col in range(5):
            f = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
            d = ImageDraw.Draw(f)
            cx, cy = 32, 36
            step = 0 if col == 0 else math.sin((col - 1) * math.pi / 2)
            d.ellipse(box(cx, cy + 16, 14, 7), fill=(10, 12, 18, 70))
            d.ellipse(box(cx, cy - 15, 9, 9), fill=skin)
            d.pieslice((23, 12, 41, 28), 180, 360, fill=hair)
            d.polygon([(32, 28), (17, 48), (24, 56), (40, 56), (47, 48)], fill=cloak)
            d.line((22, 40, 42, 40), fill=trim, width=3)
            d.line((24, 36, 18 - step * 2, 45), fill=skin, width=4)
            d.line((40, 36, 46 + step * 2, 45), fill=skin, width=4)
            d.line((28, 55, 26 - step * 2, 63), fill=trim, width=3)
            d.line((36, 55, 38 + step * 2, 63), fill=trim, width=3)
            vial_x = 46 if facing != "left" else 18
            d.rounded_rectangle((vial_x - 3, 42, vial_x + 3, 52), radius=2, fill=accent)
            img.alpha_composite(f.filter(ImageFilter.GaussianBlur(0.2)), (col * 64, row * 64))
    return img


def crow_sheet():
    img = Image.new("RGBA", (320, 256), (0, 0, 0, 0))
    for row in range(4):
        for col in range(5):
            f = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
            d = ImageDraw.Draw(f)
            cx, cy = 32, 35
            flap = 0 if col == 0 else abs(math.sin((col - 1) * math.pi / 2)) * 5
            d.ellipse(box(cx, cy + 16, 14, 7), fill=(10, 12, 18, 70))
            d.ellipse(box(cx, cy, 10, 13), fill=rgb("#4b5261"))
            d.polygon([(32, 16), (22, 38), (42, 38)], fill=rgb("#cbd2de"))
            d.ellipse(box(cx - 11, cy + 2, 8 + flap, 6), fill=rgb("#3b404a"))
            d.ellipse(box(cx + 11, cy + 2, 8 + flap, 6), fill=rgb("#3b404a"))
            d.line((28, 48, 26 - flap * 0.3, 60), fill=rgb("#e3d38d"), width=2)
            d.line((36, 48, 38 + flap * 0.3, 60), fill=rgb("#e3d38d"), width=2)
            img.alpha_composite(f, (col * 64, row * 64))
    return img


def station(kind, size):
    w, h = size
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    cx, cy = w / 2, h / 2 + 4
    d.ellipse(box(cx, cy + h * 0.24, w * 0.22, h * 0.1), fill=(10, 12, 18, 70))
    if "cauldron" in kind:
        d.ellipse(box(cx, cy + 10, 34, 28), fill=rgb("#34343e"))
        d.ellipse(box(cx, cy + 2, 24, 16), fill=rgb("#53ddd0"))
        d.arc((cx - 30, cy - 6, cx + 30, cy + 34), 190, 350, fill=rgb("#c6905d"), width=5)
    elif "rest_bed" in kind:
        d.rounded_rectangle((cx - 36, cy - 18, cx + 36, cy + 18), radius=10, fill=rgb("#7b5c41"))
        d.rounded_rectangle((cx - 28, cy - 12, cx + 24, cy + 12), radius=8, fill=rgb("#d2c4a0"))
        d.rounded_rectangle((cx + 12, cy - 10, cx + 28, cy + 4), radius=6, fill=rgb("#ece7d6"))
        d.polygon([(cx - 26, cy - 4), (cx + 6, cy + 14), (cx - 18, cy + 14)], fill=rgb("#7a8fb7"))
        d.line((cx - 34, cy - 18, cx - 34, cy + 18), fill=rgb("#5d412b"), width=4)
        d.line((cx + 34, cy - 18, cx + 34, cy + 18), fill=rgb("#5d412b"), width=4)
    elif "still" in kind:
        d.rounded_rectangle((cx - 28, cy + 8, cx + 28, cy + 34), radius=10, fill=rgb("#4f6c57"))
        d.ellipse(box(cx - 14, cy - 10, 14, 18), fill=rgb("#dceff3"))
        d.ellipse(box(cx + 12, cy - 6, 16, 20), fill=rgb("#d8ecef"))
    elif "planter" in kind:
        d.rounded_rectangle((cx - 32, cy - 8, cx + 32, cy + 20), radius=8, fill=rgb("#8a8574"))
        d.rounded_rectangle((cx - 26, cy - 2, cx + 26, cy + 14), radius=6, fill=rgb("#5a4030"))
        for ox in [-18, -5, 8, 19]:
            d.line((cx + ox, cy + 9, cx + ox, cy - 2), fill=rgb("#7bb66d"), width=2)
            d.ellipse(box(cx + ox - 2, cy - 3, 5, 8), fill=rgb("#b8e88c"))
    elif "board" in kind:
        d.rectangle((cx - 20, cy - 26, cx + 20, cy + 14), fill=rgb("#7b5a3f"))
        d.rectangle((cx - 16, cy - 22, cx + 16, cy + 10), fill=rgb("#e8dcc5"))
        d.rectangle((cx - 4, cy + 14, cx + 4, cy + 38), fill=rgb("#6c4c33"))
    elif "habitat_moth" in kind:
        d.ellipse(box(cx, cy + 10, 34, 24), fill=rgb("#5f5b4d"))
        for ox in [-18, 0, 18]:
            d.line((cx + ox, cy - 18, cx + ox, cy + 8), fill=rgb("#cdb87d"), width=3)
    elif "habitat_slug" in kind:
        d.rounded_rectangle((cx - 32, cy - 12, cx + 32, cy + 20), radius=10, fill=rgb("#6d7d87"))
        d.rounded_rectangle((cx - 24, cy - 4, cx + 24, cy + 12), radius=8, fill=rgb("#bfe7ef"))
    elif "rune" in kind:
        d.rounded_rectangle((cx - 30, cy - 10, cx + 30, cy + 24), radius=10, fill=rgb("#544a60"))
        d.line((cx - 20, cy + 6, cx + 20, cy + 6), fill=rgb("#e2d4ff"), width=3)
        d.line((cx, cy - 8, cx, cy + 18), fill=rgb("#e2d4ff"), width=3)
    elif "archive" in kind:
        d.rounded_rectangle((cx - 34, cy - 14, cx + 34, cy + 18), radius=10, fill=rgb("#75684f"))
        for ox in [-20, 0, 20]:
            d.rectangle((cx + ox - 8, cy - 8, cx + ox + 8, cy + 8), fill=rgb("#e7d7ae"))
    elif "observatory" in kind:
        d.ellipse(box(cx, cy + 4, 34, 24), fill=rgb("#66553d"))
        d.ellipse(box(cx, cy + 4, 12, 12), fill=rgb("#dbe7ff"))
    else:
        d.rounded_rectangle((cx - 32, cy - 18, cx + 32, cy + 22), radius=10, fill=rgb("#7b5c41"))
        for ox in [-22, 0, 22]:
            d.rectangle((cx + ox - 7, cy - 10, cx + ox + 7, cy + 8), fill=rgb("#d7d0b3"))
    return img


def icon(item, size, world=False):
    w, h = size
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    cx, cy = w / 2, h / 2 + (4 if world else 0)
    if world:
        d.ellipse(box(cx, cy + 18, 16, 7), fill=(10, 12, 18, 70))
    if item in {"whisper_moss", "sunleaf", "moon_fern", "ember_root"}:
        colors = {"whisper_moss": "#74d59f", "sunleaf": "#efd05c", "moon_fern": "#b8ffd0", "ember_root": "#f18555"}
        for ox, oy in [(-10, 2), (2, -4), (12, 4), (-2, 10)]:
            d.ellipse(box(cx + ox, cy + oy, 9, 12), fill=rgb(colors[item]))
    elif item in {"arcane_dust", "lumen_dust", "starlight_shard"}:
        c = {"arcane_dust": "#a781ff", "lumen_dust": "#f1e7a3", "starlight_shard": "#dbe5ff"}[item]
        for ox, oy in [(-10, 8), (0, 0), (12, 10)]:
            d.polygon([(cx + ox, cy + oy - 10), (cx + ox - 8, cy + oy), (cx + ox, cy + oy + 10), (cx + ox + 8, cy + oy)], fill=rgb(c))
    elif item in {"mist_moth_wing", "glow_moth"}:
        c = "#b8d7ff" if item == "mist_moth_wing" else "#f2e49a"
        d.ellipse(box(cx - 12, cy + 2, 12, 16), fill=rgb(c))
        d.ellipse(box(cx + 12, cy + 2, 12, 16), fill=rgb(c))
        d.ellipse(box(cx, cy + 4, 5, 13), fill=rgb("#fff3c7"))
    elif item == "dew_slug":
        d.ellipse(box(cx, cy + 6, 18, 12), fill=rgb("#b5e8ff"))
        d.line((cx - 18, cy + 4, cx - 24, cy - 6), fill=rgb("#e3fbff"), width=2)
    elif item in {"splash_rune", "echo_rune", "delay_rune"}:
        c = {"splash_rune": "#7dc0ff", "echo_rune": "#c7b4ff", "delay_rune": "#ffc189"}[item]
        d.rounded_rectangle((cx - 18, cy - 18, cx + 18, cy + 18), radius=8, fill=rgb("#8a877f"))
        d.arc((cx - 12, cy - 10, cx + 12, cy + 12), 210, 330, fill=rgb(c), width=4)
    else:
        c = {
            "soothing_tonic": "#7fd2a5", "lantern_leak": "#c5b7ff", "rush_draught": "#ff9566",
            "healing_draught": "#83dc9e", "glow_potion": "#ccbfff", "star_lantern_elixir": "#eef1ff",
            "beastcalm_extract": "#bfe8dc", "splash_glow_potion": "#bdd4ff", "echo_healing_draught": "#b4f0be",
            "delayed_stamina_tonic": "#ffc08f", "stamina_tonic": "#ff9b62", "murky_concoction": "#7c7459",
            "dew_slime": "#c4f4ff",
        }.get(item, "#d2c1ff")
        d.rounded_rectangle((cx - 8, cy - 24, cx + 8, cy - 12), radius=3, fill=rgb("#d9dfe8", 220))
        d.rectangle((cx - 5, cy - 28, cx + 5, cy - 22), fill=rgb("#8b6a4d"))
        d.rounded_rectangle((cx - 16, cy - 14, cx + 16, cy + 22), radius=10, fill=rgb("#d9dfe8", 220))
        d.rounded_rectangle((cx - 12, cy - 2, cx + 12, cy + 18), radius=8, fill=rgb(c))
    return img


def area(name):
    colors = {
        "tower_entry": ("#5f5367", "#2b2d39"),
        "north_plains": ("#7fa66c", "#526d41"),
        "town_square": ("#7ca36d", "#55724c"),
        "rock_fields": ("#8f8773", "#535046"),
        "moonlit_forest": ("#4f6e68", "#1d2932"),
        "lake_shore": ("#73a9bd", "#385c6b"),
        "sunscar_desert": ("#c89f59", "#7f6131"),
        "tropical_rainforest": ("#4d8a67", "#1f4735"),
        "greenhouse_floor": ("#709379", "#425845"),
        "containment_floor": ("#57708c", "#2b3647"),
        "rune_workshop_floor": ("#6c5b78", "#31263a"),
        "archive_floor": ("#7d715a", "#383227"),
        "observatory_floor": ("#303f6b", "#11192a"),
    }[name]
    img = Image.new("RGBA", (1920, 1080), rgb(colors[0]))
    d = ImageDraw.Draw(img)
    for y in range(1080):
        t = y / 1079
        c1, c2 = rgb(colors[0]), rgb(colors[1])
        c = tuple(int(c1[i] * (1 - t) + c2[i] * t) for i in range(4))
        d.line((0, y, 1920, y), fill=c)
    for gy in range(0, 1080, 96):
        for gx in range(0, 1920, 96):
            d.rounded_rectangle((gx + 6, gy + 6, gx + 88, gy + 88), radius=10, outline=(0, 0, 0, 25), width=2)
    return img.filter(ImageFilter.GaussianBlur(0.2))


def fx(name):
    img = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    if name == "gather_feedback_sparkle":
        for i in range(8):
            a = i * math.pi / 4
            d.line((32, 32, 32 + math.cos(a) * 22, 32 + math.sin(a) * 22), fill=rgb("#e8fff3", 220), width=3)
    elif name == "brew_bubble_effect":
        for ox, oy, r in [(-10, 8, 9), (8, -4, 7), (14, 12, 5)]:
            d.ellipse(box(32 + ox, 32 + oy, r, r), fill=rgb("#d8fff8", 120), outline=rgb("#f3ffff", 220))
    else:
        for r, a in [(24, 90), (18, 150), (10, 220)]:
            d.ellipse(box(32, 32, r, r), outline=rgb("#b8d4ff", a), width=3)
    return img.filter(ImageFilter.GaussianBlur(0.25))


def ui_icon(name):
    img = Image.new("RGBA", (32, 32), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    c = rgb("#f0e2b8")
    if name in {"routes", "route_restored"}:
        d.arc((5, 8, 27, 28), 210, 20, fill=c, width=3)
    elif name in {"notes", "journal_note"}:
        d.rounded_rectangle((7, 5, 24, 27), radius=4, outline=c, width=3)
    elif name in {"brews", "recipe_logged"}:
        d.rounded_rectangle((8, 8, 24, 26), radius=6, outline=c, width=3)
        d.rectangle((12, 4, 20, 10), fill=c)
    elif name == "greenhouse":
        d.rounded_rectangle((5, 8, 27, 26), radius=5, outline=c, width=3)
        d.line((16, 8, 16, 26), fill=c, width=2)
    elif name == "rapport":
        d.ellipse((7, 6, 25, 24), outline=c, width=3)
    else:
        d.ellipse((6, 6, 26, 26), outline=c, width=3)
        d.line((12, 16, 15, 20), fill=c, width=3)
        d.line((15, 20, 22, 10), fill=c, width=3)
    return img


def main():
    char_palettes = {
        "player_tower_alchemist": ["#4f6fa8", "#88b8ff", "#7e4f39", "#f0d3b6", "#64f0d0"],
        "mira_apothecary": ["#cb7e71", "#f3d2a7", "#8f5b40", "#f1d2b6", "#ffe2a4"],
        "rowan_herbalist": ["#5c7d4b", "#9fcb8a", "#5a4333", "#e0c2aa", "#f2d57a"],
        "mayor_elric": ["#7a5a3a", "#d9b870", "#61452f", "#e1c4a7", "#d8bc7c"],
        "ione_archivist": ["#617ea2", "#d5d7cf", "#58453a", "#e4d0bc", "#b0cfff"],
        "brin_groundskeeper": ["#6b5a43", "#7d9c63", "#5c4738", "#ddc4a6", "#d6b083"],
        "lyra_keeper": ["#8aa7c4", "#dfe9ff", "#6a5446", "#ebd8c8", "#bde6e8"],
    }
    for req in REQ["sprite_requirements"]["player"] + REQ["sprite_requirements"]["npcs"]:
        save(sheet(char_palettes[req["id"]]), Path("characters") / f'{req["id"]}.png')
    save(crow_sheet(), Path("characters/crow_guide.png"))
    for req in REQ["sprite_requirements"]["stations"]:
        save(station(req["id"], req["image_size_px"]), Path("stations") / f'{req["id"]}.png')
    for req in REQ["sprite_requirements"]["items_and_gatherables"]:
        size = req.get("icon_size_px") or req.get("world_sprite_px")
        save(icon(req["id"], size), Path("items/icons") / f'{req["id"]}.png')
        if req["type"] == "icon_and_world_node":
            save(icon(req["id"], req["world_sprite_px"], True), Path("items/world") / f'{req["id"]}.png')
    for req in REQ["sprite_requirements"]["areas"]:
        save(area(req["id"]), Path("areas") / f'{req["id"]}.png')
    for name in JOURNAL:
        save(ui_icon(name), Path("ui/journal_tabs") / f"{name}.png")
    for name in TOASTS:
        save(ui_icon(name), Path("ui/toasts") / f"{name}.png")
    for req in REQ["sprite_requirements"]["ui_and_effects"]:
        if req["type"] == "effect_sprite":
            save(fx(req["id"]), Path("effects") / f'{req["id"]}.png')


if __name__ == "__main__":
    main()
