import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]

class LauncherTests(unittest.TestCase):
    def test_launch_paths_are_not_shell_commands(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            libexec = root / "lib exec"
            binary = libexec / "omarchy-spacecadet/omarchy-spacecadet-game"
            binary.parent.mkdir(parents=True)
            binary.write_text("#!/usr/bin/env python3\nimport os,sys\nprint(os.getcwd())\nprint(repr(sys.argv[1:]))\n")
            binary.chmod(0o755)
            launcher = root / "launcher"
            launcher.write_text((ROOT / "Platform/Linux/omarchy-spacecadet.in").read_text().replace("@CMAKE_INSTALL_FULL_LIBEXECDIR@", str(libexec)))
            data = root / "data $(touch WRONG)"
            data.mkdir()
            (data / "PINBALL.DAT").write_bytes(b"test fixture, not real game data")
            env = dict(os.environ, XDG_CONFIG_HOME=str(root / "config"))
            result = subprocess.run(["python3", str(launcher), "--data-dir", str(data), "-sw"], env=env, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn(str(data), result.stdout)
            self.assertIn("'-sw'", result.stdout)
            saved = root / "config/omarchy-spacecadet/data-directory"
            self.assertEqual(saved.read_text().strip(), str(data))
            again = subprocess.run(["python3", str(launcher), "--classic"], env=env, capture_output=True, text=True)
            self.assertEqual(again.returncode, 0, again.stderr)
            invalid = subprocess.run(["python3", str(launcher), "--data-dir", str(root / "missing")], env=env, capture_output=True, text=True)
            self.assertEqual(invalid.returncode, 2)
            self.assertFalse((data / "WRONG").exists())
            native = binary.with_name("omarchy-pinball")
            native.write_text("#!/usr/bin/env python3\nprint('original table ready')\n")
            native.chmod(0o755)
            result = subprocess.run(["python3", str(launcher)], env=env, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("original table ready", result.stdout)

if __name__ == "__main__":
    unittest.main()
