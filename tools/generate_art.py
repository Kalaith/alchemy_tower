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
    style_suffix = None
    base_item = item
    for suffix in ["plains", "quarry", "forest", "lake", "desert", "rainforest"]:
        token = f"_{suffix}"
        if item.endswith(token):
            base_item = item[: -len(token)]
            style_suffix = suffix
            break
    if world:
        d.ellipse(box(cx, cy + 18, 16, 7), fill=(10, 12, 18, 70))
    if item in {
        "whisper_moss",
        "sunleaf",
        "moon_fern",
        "ember_root",
        "field_bloom",
        "quarry_lichen",
        "reedflower",
        "sunspike",
        "rain_orchid",
    } or base_item in {
        "whisper_moss",
        "sunleaf",
        "moon_fern",
        "ember_root",
        "field_bloom",
        "quarry_lichen",
        "reedflower",
        "sunspike",
        "rain_orchid",
    }:
        colors = {
            "whisper_moss": "#74d59f",
            "sunleaf": "#efd05c",
            "moon_fern": "#b8ffd0",
            "ember_root": "#f18555",
            "field_bloom": "#f6d784",
            "quarry_lichen": "#a8d2ba",
            "reedflower": "#d6f1ff",
            "sunspike": "#ffc864",
            "rain_orchid": "#f0a7c8",
        }
        for ox, oy in [(-10, 2), (2, -4), (12, 4), (-2, 10)]:
            d.ellipse(box(cx + ox, cy + oy, 9, 12), fill=rgb(colors[base_item]))
        if base_item == "field_bloom":
            d.ellipse(box(cx, cy - 2, 7, 7), fill=rgb("#fff2ba"))
        elif base_item == "quarry_lichen":
            d.rectangle((cx - 18, cy + 8, cx + 18, cy + 18), fill=rgb("#6b746d"))
        elif base_item == "reedflower":
            for rx in [-12, 0, 12]:
                d.line((cx + rx, cy + 18, cx + rx - 2, cy - 10), fill=rgb("#9fc58a"), width=2)
        elif base_item == "sunspike":
            d.polygon([(cx, cy - 18), (cx - 12, cy + 16), (cx + 12, cy + 16)], fill=rgb("#fff0aa"))
        elif base_item == "rain_orchid":
            d.ellipse(box(cx, cy + 2, 8, 10), fill=rgb("#fff4c8"))
    elif base_item in {"arcane_dust", "lumen_dust", "starlight_shard"}:
        c = {"arcane_dust": "#a781ff", "lumen_dust": "#f1e7a3", "starlight_shard": "#dbe5ff"}[base_item]
        for ox, oy in [(-10, 8), (0, 0), (12, 10)]:
            d.polygon([(cx + ox, cy + oy - 10), (cx + ox - 8, cy + oy), (cx + ox, cy + oy + 10), (cx + ox + 8, cy + oy)], fill=rgb(c))
    elif base_item in {"mist_moth_wing", "glow_moth"}:
        c = "#b8d7ff" if base_item == "mist_moth_wing" else "#f2e49a"
        d.ellipse(box(cx - 12, cy + 2, 12, 16), fill=rgb(c))
        d.ellipse(box(cx + 12, cy + 2, 12, 16), fill=rgb(c))
        d.ellipse(box(cx, cy + 4, 5, 13), fill=rgb("#fff3c7"))
    elif base_item == "dew_slug":
        d.ellipse(box(cx, cy + 6, 18, 12), fill=rgb("#b5e8ff"))
        d.line((cx - 18, cy + 4, cx - 24, cy - 6), fill=rgb("#e3fbff"), width=2)
    elif base_item in {"splash_rune", "echo_rune", "delay_rune"}:
        c = {"splash_rune": "#7dc0ff", "echo_rune": "#c7b4ff", "delay_rune": "#ffc189"}[base_item]
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
    if world and style_suffix:
        if style_suffix == "plains":
            d.line((cx - 20, cy + 18, cx + 20, cy + 6), fill=rgb("#fff4bf", 140), width=3)
        elif style_suffix == "quarry":
            d.rectangle((cx - 18, cy + 10, cx + 18, cy + 18), fill=rgb("#70675f", 170))
        elif style_suffix == "forest":
            d.ellipse(box(cx, cy + 4, 20, 14), outline=rgb("#c3ffd5", 140), width=2)
        elif style_suffix == "lake":
            d.arc((cx - 22, cy - 2, cx + 18, cy + 26), 180, 340, fill=rgb("#d8f4ff", 160), width=3)
        elif style_suffix == "desert":
            d.arc((cx - 20, cy + 4, cx + 20, cy + 26), 200, 340, fill=rgb("#ffe4a6", 150), width=3)
        elif style_suffix == "rainforest":
            d.line((cx - 18, cy + 20, cx + 12, cy - 10), fill=rgb("#d4ffe6", 140), width=3)
    return img


def area(name):
    colors = {
        "tower_entry": ("#5f5367", "#2b2d39"),
        "north_plains": ("#8fb96f", "#587245"),
        "town_square": ("#7ca36d", "#55724c"),
        "rock_fields": ("#958978", "#585148"),
        "moonlit_forest": ("#52756a", "#1d2932"),
        "lake_shore": ("#85bdd0", "#446d79"),
        "sunscar_desert": ("#d0aa69", "#8a6537"),
        "tropical_rainforest": ("#5a9d74", "#1f4735"),
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
    if name == "north_plains":
        for y in range(100, 1080, 140):
            d.arc((120, y - 60, 1780, y + 160), 195, 345, fill=rgb("#d9c38c", 120), width=18)
        for x in range(120, 1820, 220):
            d.ellipse((x, 150 + (x % 160), x + 90, 210 + (x % 160)), fill=rgb("#e3d48f", 120))
    elif name == "rock_fields":
        for x, y, w, h in [(140, 220, 520, 150), (840, 180, 580, 170), (520, 640, 700, 180)]:
            d.rounded_rectangle((x, y, x + w, y + h), radius=26, fill=rgb("#71685c", 220))
            d.rounded_rectangle((x + 24, y + 18, x + w - 24, y + 54), radius=20, fill=rgb("#b0a18e", 180))
        for x, y in [(420, 420), (980, 470), (790, 740)]:
            d.polygon([(x, y - 34), (x - 28, y), (x, y + 34), (x + 28, y)], fill=rgb("#b79df9", 90))
    elif name == "moonlit_forest":
        for x in range(120, 1840, 220):
            d.ellipse((x, 90 + (x % 140), x + 240, 300 + (x % 140)), fill=rgb("#355742", 120))
            d.rectangle((x + 90, 260 + (x % 140), x + 118, 460 + (x % 140)), fill=rgb("#4b3429", 160))
        d.ellipse((700, 460, 1120, 760), fill=rgb("#b2d6bd", 70))
    elif name == "lake_shore":
        d.rounded_rectangle((90, 280, 1080, 980), radius=160, fill=rgb("#5fa0c0", 170))
        d.rounded_rectangle((260, 360, 1000, 940), radius=140, fill=rgb("#7cc3de", 160))
        for x in range(180, 1080, 70):
            d.line((x, 340, x - 14, 200), fill=rgb("#d9d59a", 120), width=6)
    elif name == "sunscar_desert":
        for y in range(180, 1080, 170):
            d.arc((60, y - 80, 1860, y + 160), 190, 350, fill=rgb("#ecd08c", 150), width=30)
        for x, y in [(420, 280), (1140, 360), (860, 760)]:
            d.polygon([(x, y - 70), (x - 55, y + 32), (x + 42, y + 64)], fill=rgb("#9c7343", 180))
    elif name == "tropical_rainforest":
        for x in range(80, 1860, 180):
            d.ellipse((x, 70 + (x % 110), x + 220, 280 + (x % 110)), fill=rgb("#2f6f4d", 150))
            d.rectangle((x + 92, 250 + (x % 110), x + 120, 500 + (x % 110)), fill=rgb("#5b402f", 170))
        for x, y in [(380, 740), (980, 700), (1420, 620)]:
            d.arc((x - 120, y - 60, x + 120, y + 90), 180, 340, fill=rgb("#8d5f3f", 170), width=16)
    else:
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
        if req["type"] in {"icon_and_world_node", "inventory_icon"}:
            size = req.get("icon_size_px") or req.get("world_sprite_px")
            save(icon(req["id"], size), Path("items/icons") / f'{req["id"]}.png')
        if req["type"] in {"icon_and_world_node", "world_node_variant"}:
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
