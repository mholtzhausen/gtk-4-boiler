/**
 * Tests for brainstorm-mode utilities.
 *
 * Run with: node --experimental-strip-types --test .pi/extensions/brainstorm-mode/test-utils.ts
 */

import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { isSafeCommand } from "./utils.js";

void describe("isSafeCommand", () => {
	void it("allows read-only commands", () => {
		assert.equal(isSafeCommand("cat file.txt"), true);
		assert.equal(isSafeCommand("head -n 10 file.txt"), true);
		assert.equal(isSafeCommand("tail -f log.txt"), true);
		assert.equal(isSafeCommand("less README.md"), true);
		assert.equal(isSafeCommand("grep -r 'pattern' src/"), true);
		assert.equal(isSafeCommand("find . -name '*.ts'"), true);
		assert.equal(isSafeCommand("ls -la"), true);
		assert.equal(isSafeCommand("pwd"), true);
		assert.equal(isSafeCommand("tree src/"), true);
		assert.equal(isSafeCommand("git status"), true);
		assert.equal(isSafeCommand("git log --oneline"), true);
		assert.equal(isSafeCommand("git diff HEAD"), true);
		assert.equal(isSafeCommand("rg 'TODO'"), true);
		assert.equal(isSafeCommand("fd '*.ts'"), true);
	});

	void it("blocks destructive commands", () => {
		assert.equal(isSafeCommand("rm file.txt"), false);
		assert.equal(isSafeCommand("mv file.txt new.txt"), false);
		assert.equal(isSafeCommand("cp file.txt new.txt"), false);
		assert.equal(isSafeCommand("mkdir newdir"), false);
		assert.equal(isSafeCommand("touch file.txt"), false);
		assert.equal(isSafeCommand("chmod +x script.sh"), false);
		assert.equal(isSafeCommand("sudo rm -rf /"), false);
		assert.equal(isSafeCommand("git add ."), false);
		assert.equal(isSafeCommand("git commit -m 'msg'"), false);
		assert.equal(isSafeCommand("git push"), false);
	});

	void it("blocks package install commands", () => {
		assert.equal(isSafeCommand("npm install"), false);
		assert.equal(isSafeCommand("npm install react"), false);
		assert.equal(isSafeCommand("yarn add lodash"), false);
		assert.equal(isSafeCommand("pip install flask"), false);
		assert.equal(isSafeCommand("apt-get install gcc"), false);
		assert.equal(isSafeCommand("brew install node"), false);
	});

	void it("blocks editors", () => {
		assert.equal(isSafeCommand("vim file.ts"), false);
		assert.equal(isSafeCommand("nano file.ts"), false);
		assert.equal(isSafeCommand("emacs file.ts"), false);
		assert.equal(isSafeCommand("code ."), false);
	});

	void it("blocks output redirection", () => {
		assert.equal(isSafeCommand("echo hello > file.txt"), false);
		assert.equal(isSafeCommand("echo hello >> file.txt"), false);
	});

	void it("allows curl and wget", () => {
		assert.equal(isSafeCommand("curl https://example.com"), true);
		assert.equal(isSafeCommand("wget -O - https://example.com"), true);
	});

	void it("allows system info commands", () => {
		assert.equal(isSafeCommand("uname -a"), true);
		assert.equal(isSafeCommand("whoami"), true);
		assert.equal(isSafeCommand("date"), true);
		assert.equal(isSafeCommand("uptime"), true);
		assert.equal(isSafeCommand("ps aux"), true);
	});
});
