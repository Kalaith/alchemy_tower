import math
import random
import wave
from array import array
from pathlib import Path

SAMPLE_RATE = 48_000
OUTPUT_DIR = Path("assets/generated/audio")


def clamp(sample: float) -> int:
    return max(-32767, min(32767, int(sample * 32767)))


def write_wav(path: Path, samples: list[float]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    pcm = array("h", (clamp(sample) for sample in samples))
    with wave.open(str(path), "wb") as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(SAMPLE_RATE)
        wav_file.writeframes(pcm.tobytes())


def envelope(progress: float, attack: float, decay: float) -> float:
    if progress <= attack:
        return progress / max(attack, 1e-6)
    tail = (1.0 - progress) / max(decay, 1e-6)
    return max(0.0, min(1.0, tail))


def add_sine(buffer: list[float], frequency: float, volume: float, attack: float, decay: float) -> None:
    length = len(buffer)
    for index in range(length):
        progress = index / max(length - 1, 1)
        amp = envelope(progress, attack, decay)
        buffer[index] += math.sin(2.0 * math.pi * frequency * progress * (length / SAMPLE_RATE)) * volume * amp


def add_noise(
    buffer: list[float],
    rng: random.Random,
    volume: float,
    attack: float,
    decay: float,
    highpass_strength: float = 0.0,
) -> None:
    previous = 0.0
    length = len(buffer)
    for index in range(length):
        progress = index / max(length - 1, 1)
        amp = envelope(progress, attack, decay)
        value = rng.uniform(-1.0, 1.0)
        filtered = value - previous * highpass_strength
        previous = value
        buffer[index] += filtered * volume * amp


def normalize(buffer: list[float], target: float = 0.9) -> list[float]:
    peak = max((abs(sample) for sample in buffer), default=1.0)
    if peak <= 0.0001:
        return buffer
    scale = target / peak
    return [sample * scale for sample in buffer]


def footstep_stone(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.14 + rng.uniform(0.0, 0.03)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.26, 0.01, 0.25, highpass_strength=0.42)
    add_sine(buffer, 120 + rng.uniform(-20, 15), 0.28, 0.005, 0.18)
    add_sine(buffer, 210 + rng.uniform(-25, 25), 0.12, 0.01, 0.22)
    return normalize(buffer, 0.72)


def footstep_dirt(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.16 + rng.uniform(0.0, 0.04)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.32, 0.01, 0.32, highpass_strength=0.18)
    add_sine(buffer, 95 + rng.uniform(-15, 15), 0.14, 0.01, 0.28)
    return normalize(buffer, 0.7)


def footstep_greenhouse(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.18 + rng.uniform(0.0, 0.04)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.24, 0.012, 0.34, highpass_strength=0.15)
    add_sine(buffer, 140 + rng.uniform(-18, 18), 0.11, 0.008, 0.24)
    add_sine(buffer, 520 + rng.uniform(-60, 60), 0.05, 0.03, 0.18)
    return normalize(buffer, 0.66)


def gather_pickup(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.24 + rng.uniform(0.0, 0.04)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.22, 0.015, 0.42, highpass_strength=0.08)
    add_sine(buffer, 680 + rng.uniform(-40, 50), 0.12, 0.008, 0.2)
    add_sine(buffer, 1020 + rng.uniform(-60, 60), 0.06, 0.01, 0.16)
    return normalize(buffer, 0.7)


def alchemy_open(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.4 + rng.uniform(0.0, 0.06)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_sine(buffer, 220 + rng.uniform(-10, 12), 0.16, 0.02, 0.6)
    add_sine(buffer, 330 + rng.uniform(-15, 18), 0.11, 0.03, 0.55)
    add_sine(buffer, 495 + rng.uniform(-20, 20), 0.08, 0.05, 0.42)
    add_noise(buffer, rng, 0.04, 0.02, 0.18, highpass_strength=0.4)
    return normalize(buffer, 0.62)


def alchemy_stir(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.22 + rng.uniform(0.0, 0.04)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.18, 0.01, 0.34, highpass_strength=0.22)
    add_sine(buffer, 260 + rng.uniform(-20, 20), 0.12, 0.02, 0.3)
    add_sine(buffer, 530 + rng.uniform(-35, 40), 0.05, 0.015, 0.22)
    return normalize(buffer, 0.64)


def brew_success(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.62 + rng.uniform(0.0, 0.05)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_sine(buffer, 392 + rng.uniform(-8, 8), 0.16, 0.02, 0.7)
    add_sine(buffer, 494 + rng.uniform(-8, 8), 0.12, 0.04, 0.65)
    add_sine(buffer, 588 + rng.uniform(-10, 10), 0.08, 0.06, 0.55)
    add_noise(buffer, rng, 0.025, 0.04, 0.2, highpass_strength=0.5)
    return normalize(buffer, 0.66)


def brew_collapse(seed: int) -> list[float]:
    rng = random.Random(seed)
    duration = 0.5 + rng.uniform(0.0, 0.06)
    buffer = [0.0] * int(SAMPLE_RATE * duration)
    add_noise(buffer, rng, 0.28, 0.015, 0.5, highpass_strength=0.1)
    add_sine(buffer, 180 + rng.uniform(-12, 10), 0.1, 0.01, 0.3)
    add_sine(buffer, 110 + rng.uniform(-12, 12), 0.12, 0.02, 0.45)
    return normalize(buffer, 0.68)


ASSETS = {
    "footstep_stone": (6, footstep_stone),
    "footstep_dirt_path": (6, footstep_dirt),
    "footstep_greenhouse": (5, footstep_greenhouse),
    "gather_herb_pickup": (5, gather_pickup),
    "alchemy_station_open": (2, alchemy_open),
    "alchemy_stir": (4, alchemy_stir),
    "brew_success": (3, brew_success),
    "brew_collapse": (3, brew_collapse),
}


def main() -> None:
    for name, (count, generator) in ASSETS.items():
        for index in range(1, count + 1):
            write_wav(OUTPUT_DIR / f"{name}_{index}.wav", generator(index))


if __name__ == "__main__":
    main()
