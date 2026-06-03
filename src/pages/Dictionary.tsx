import { useEffect, useState } from "react";
import { Plus, Trash2, Wand2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface DictionaryEntry {
  id: string;
  spoken_phrase: string;
  replacement: string;
  category: string | null;
  enabled: boolean;
  use_count: number;
}

export default function Dictionary() {
  const [entries, setEntries] = useState<DictionaryEntry[]>([]);
  const [spoken, setSpoken] = useState("");
  const [replacement, setReplacement] = useState("");
  const [category, setCategory] = useState("");

  const fetchEntries = async () => {
    try {
      const result = await invoke<DictionaryEntry[]>("get_dictionary");
      setEntries(result);
    } catch {
      // ignore
    }
  };

  useEffect(() => {
    fetchEntries();
  }, []);

  const handleAdd = async () => {
    if (!spoken.trim() || !replacement.trim()) return;
    try {
      await invoke("add_dictionary_entry", {
        spokenPhrase: spoken.trim(),
        replacement: replacement.trim(),
        category: category.trim() || null,
      });
      setSpoken("");
      setReplacement("");
      setCategory("");
      fetchEntries();
    } catch {
      // ignore
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await invoke("delete_dictionary_entry", { id });
      setEntries(entries.filter((e) => e.id !== id));
    } catch {
      // ignore
    }
  };

  const handleSeed = async () => {
    try {
      await invoke("seed_developer_dictionary");
      fetchEntries();
    } catch {
      // ignore
    }
  };

  const categories = [...new Set(entries.map((e) => e.category).filter(Boolean))];

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold">Custom Dictionary</h1>
          <p className="text-sm text-surface-500 mt-1">
            Map spoken phrases to exact replacements for technical terms.
          </p>
        </div>
        <button onClick={handleSeed} className="btn-secondary flex items-center gap-2">
          <Wand2 className="w-4 h-4" />
          Add Dev Terms
        </button>
      </div>

      <div className="card">
        <div className="grid grid-cols-[1fr_1fr_auto_auto] gap-3 items-end">
          <div>
            <label className="text-xs font-medium text-surface-600 dark:text-surface-400 mb-1 block">
              Spoken phrase
            </label>
            <input
              type="text"
              value={spoken}
              onChange={(e) => setSpoken(e.target.value)}
              placeholder="e.g., next js"
              className="input"
            />
          </div>
          <div>
            <label className="text-xs font-medium text-surface-600 dark:text-surface-400 mb-1 block">
              Replacement
            </label>
            <input
              type="text"
              value={replacement}
              onChange={(e) => setReplacement(e.target.value)}
              placeholder="e.g., Next.js"
              className="input"
            />
          </div>
          <div>
            <label className="text-xs font-medium text-surface-600 dark:text-surface-400 mb-1 block">
              Category
            </label>
            <input
              type="text"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              placeholder="optional"
              className="input w-28"
            />
          </div>
          <button
            onClick={handleAdd}
            disabled={!spoken.trim() || !replacement.trim()}
            className="btn-primary flex items-center gap-1"
          >
            <Plus className="w-4 h-4" />
            Add
          </button>
        </div>
      </div>

      {entries.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-surface-500">No dictionary entries yet.</p>
          <p className="text-xs text-surface-400 mt-1">
            Add custom replacements or seed with common developer terms.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {categories.map((cat) => (
            <div key={cat}>
              <h3 className="text-xs font-semibold text-surface-500 uppercase tracking-wider mb-2">
                {cat}
              </h3>
              <div className="space-y-1">
                {entries
                  .filter((e) => e.category === cat)
                  .map((entry) => (
                    <DictionaryRow
                      key={entry.id}
                      entry={entry}
                      onDelete={handleDelete}
                    />
                  ))}
              </div>
            </div>
          ))}

          {entries.filter((e) => !e.category).length > 0 && (
            <div>
              <h3 className="text-xs font-semibold text-surface-500 uppercase tracking-wider mb-2">
                Uncategorized
              </h3>
              <div className="space-y-1">
                {entries
                  .filter((e) => !e.category)
                  .map((entry) => (
                    <DictionaryRow
                      key={entry.id}
                      entry={entry}
                      onDelete={handleDelete}
                    />
                  ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function DictionaryRow({
  entry,
  onDelete,
}: {
  entry: DictionaryEntry;
  onDelete: (id: string) => void;
}) {
  return (
    <div className="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-surface-50 dark:hover:bg-surface-900 group">
      <span className="text-sm text-surface-600 dark:text-surface-400 w-40 truncate">
        "{entry.spoken_phrase}"
      </span>
      <span className="text-surface-400">→</span>
      <span className="text-sm font-mono font-medium flex-1 truncate">
        {entry.replacement}
      </span>
      <span className="text-xs text-surface-400">
        {entry.use_count > 0 && `${entry.use_count}×`}
      </span>
      <button
        onClick={() => onDelete(entry.id)}
        className="opacity-0 group-hover:opacity-100 p-1 hover:text-red-500 transition-all"
      >
        <Trash2 className="w-3.5 h-3.5" />
      </button>
    </div>
  );
}
