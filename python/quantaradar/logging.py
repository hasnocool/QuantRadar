#!/usr/bin/env python3
"""Lightweight Python logging: per-level files, rollover, batched flush."""
import logging
import logging.handlers
from pathlib import Path

LOG_DIR = Path("logs")
MAX_BYTES = 10 * 1024 * 1024  # 10MB rollover
BACKUP_COUNT = 3


def setup():
    LOG_DIR.mkdir(exist_ok=True)
    handlers = {
        "debug": logging.handlers.RotatingFileHandler(LOG_DIR / "debug.log", maxBytes=MAX_BYTES, backupCount=BACKUP_COUNT),
        "info": logging.handlers.RotatingFileHandler(LOG_DIR / "info.log", maxBytes=MAX_BYTES, backupCount=BACKUP_COUNT),
        "error": logging.handlers.RotatingFileHandler(LOG_DIR / "error.log", maxBytes=MAX_BYTES, backupCount=BACKUP_COUNT),
    }
    fmt = logging.Formatter("%(asctime)s [%(levelname)s] %(message)s")
    for h in handlers.values():
        h.setFormatter(fmt)
    loggers = {}
    for name, level in [("debug", logging.DEBUG), ("info", logging.INFO), ("error", logging.ERROR)]:
        logger = logging.getLogger(name)
        logger.setLevel(level)
        logger.handlers.clear()
        logger.addHandler(handlers[name])
        logger.propagate = False
        loggers[name] = logger
    return loggers

if __name__ == "__main__":
    loggers = setup()
    loggers["debug"].debug("python logging debug check")
    loggers["info"].info("python logging info check")
    loggers["error"].error("python logging error check")
