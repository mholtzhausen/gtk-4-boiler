/**
 * Brainstorm Mode Extension
 *
 * Interactive exploration mode for brainstorming and design discussions.
 * The LLM uses the `question` tool to ask the user for decisions,
 * preferences, and clarifications interactively — instead of outputting
 * text walls full of questions.
 *
 * Features:
 * - /brainstorm command or Ctrl+Alt+B to toggle
 * - Bash restricted to allowlisted read-only commands
 * - `question` tool available for the LLM to get interactive user input
 * - System prompt instructs the LLM to use `question` for *every* decision point
 * - Footer status indicator when active
 * - Session state persistence
 */

import type { AgentMessage } from "@mariozechner/pi-agent-core";
import type { AssistantMessage, TextContent } from "@mariozechner/pi-ai";
import type { ExtensionAPI, ExtensionContext } from "@mariozechner/pi-coding-agent";
import { Key } from "@mariozechner/pi-tui";
import { registerQuestionTool } from "./question.js";
import { isSafeCommand, type BrainstormState } from "./utils.js";

// Tools available in brainstorm mode
const BRAINSTORM_TOOLS = ["read", "bash", "grep", "find", "ls", "question"];
const NORMAL_TOOLS = ["read", "bash", "edit", "write"];

// Type guard for assistant messages
function isAssistantMessage(m: AgentMessage): m is AssistantMessage {
	return m.role === "assistant" && Array.isArray(m.content);
}

// Extract text content from an assistant message
function getTextContent(message: AssistantMessage): string {
	return message.content
		.filter((block): block is TextContent => block.type === "text")
		.map((block) => block.text)
		.join("\n");
}

export default function brainstormModeExtension(pi: ExtensionAPI): void {
	let brainstormEnabled = false;

	// ── Register the `question` tool first ──────────────────────────
	registerQuestionTool(pi);

	// ── Flag ────────────────────────────────────────────────────────
	pi.registerFlag("brainstorm", {
		description: "Start in brainstorm mode (interactive exploration with user polling)",
		type: "boolean",
		default: false,
	});

	// ── Helpers ─────────────────────────────────────────────────────
	function updateStatus(ctx: ExtensionContext): void {
		if (brainstormEnabled) {
			ctx.ui.setStatus(
				"brainstorm-mode",
				ctx.ui.theme.fg("accent", "💡 brainstorm"),
			);
		} else {
			ctx.ui.setStatus("brainstorm-mode", undefined);
		}
	}

	function toggleBrainstorm(ctx: ExtensionContext): void {
		brainstormEnabled = !brainstormEnabled;

		if (brainstormEnabled) {
			pi.setActiveTools(BRAINSTORM_TOOLS);
			ctx.ui.notify(
				`Brainstorm mode enabled. Tools: ${BRAINSTORM_TOOLS.join(", ")}`,
			);
		} else {
			pi.setActiveTools(NORMAL_TOOLS);
			ctx.ui.notify("Brainstorm mode disabled. Full access restored.");
		}
		updateStatus(ctx);
	}

	function persistState(): void {
		pi.appendEntry("brainstorm-mode", {
			enabled: brainstormEnabled,
		} satisfies BrainstormState);
	}

	// ── Commands ────────────────────────────────────────────────────
	pi.registerCommand("brainstorm", {
		description: "Toggle brainstorm mode (interactive exploration with user polling)",
		handler: async (_args, ctx) => toggleBrainstorm(ctx),
	});

	// ── Keyboard shortcut ───────────────────────────────────────────
	pi.registerShortcut(Key.ctrlAlt("b"), {
		description: "Toggle brainstorm mode",
		handler: async (ctx) => toggleBrainstorm(ctx),
	});

	// ── Block destructive bash commands ─────────────────────────────
	pi.on("tool_call", async (event) => {
		if (!brainstormEnabled || event.toolName !== "bash") return;

		const command = event.input.command as string;
		if (!isSafeCommand(command)) {
			return {
				block: true,
				reason:
					`Brainstorm mode: command blocked (not allowlisted). ` +
					`Use /brainstorm to disable brainstorm mode first.\nCommand: ${command}`,
			};
		}
	});

	// ── Filter stale brainstorm mode context ────────────────────────
	pi.on("context", async (event) => {
		if (brainstormEnabled) return;

		return {
			messages: event.messages.filter((m) => {
				const msg = m as AgentMessage & { customType?: string };
				if (msg.customType === "brainstorm-mode-context") return false;
				if (msg.role !== "user") return true;

				const content = msg.content;
				if (typeof content === "string") {
					return !content.includes("[BRAINSTORM MODE ACTIVE]");
				}
				if (Array.isArray(content)) {
					return !content.some(
						(c) =>
							c.type === "text" &&
							(c as TextContent).text?.includes("[BRAINSTORM MODE ACTIVE]"),
					);
				}
				return true;
			}),
		};
	});

	// ── System prompt injection ─────────────────────────────────────
	pi.on("before_agent_start", async () => {
		if (!brainstormEnabled) return;

		return {
			message: {
				customType: "brainstorm-mode-context",
				content: `[BRAINSTORM MODE ACTIVE]
You are in brainstorm mode — an interactive exploration mode for discussing ideas,
design decisions, requirements gathering, and planning.

Restrictions:
- You can only use: read, bash, grep, find, ls, question
- You CANNOT use: edit, write (file modifications are disabled)
- Bash is restricted to an allowlist of read-only commands

CRITICAL RULE — Use the 'question' tool for EVERY decision point:
- Whenever you need the user's opinion, preference, choice, or clarification,
  call the 'question' tool. Do NOT write text questions in your response.
- Ask one question at a time. Do not bundle multiple questions into one tool call.
- Provide clear, specific options for the user to choose from.
- Use the "Type something." option when the user might want a custom answer.
- After the user answers, proceed with the next step or question.
- If you need to confirm something, use the 'question' tool with Yes/No options.

Examples of when to use the 'question' tool:
  "What framework should we use?" → question({ question: "...", options: [...] })
  "Do you want me to explore this further?" → question({ question: "...", options: [...] })
  "Which approach do you prefer?" → question({ question: "...", options: [...] })

Do NOT attempt to make changes — just discuss, explore, and gather input.`,
				display: false,
			},
		};
	});

	// ── Restore state on session start/resume ───────────────────────
	pi.on("session_start", async (_event, ctx) => {
		if (pi.getFlag("brainstorm") === true) {
			brainstormEnabled = true;
		}

		const entries = ctx.sessionManager.getEntries();
		const brainstormEntry = entries
			.filter(
				(e: { type: string; customType?: string }) =>
					e.type === "custom" && e.customType === "brainstorm-mode",
			)
			.pop() as
			| { data?: BrainstormState }
			| undefined;

		if (brainstormEntry?.data) {
			brainstormEnabled = brainstormEntry.data.enabled ?? brainstormEnabled;
		}

		if (brainstormEnabled) {
			pi.setActiveTools(BRAINSTORM_TOOLS);
		}
		updateStatus(ctx);
	});
}
