"""Network sub-package (remote build architecture)."""
from natlab.network.server import BuildServer
from natlab.network.client import BuildClient

__all__ = ["BuildServer", "BuildClient"]
