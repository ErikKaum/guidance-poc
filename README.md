

Create venv and install nuitka

```bash
cd guidance
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

Build the python script as a standalone executable

```bash
python -m nuitka --output-filename=guidance --follow-imports guidance.py
```

Example usage
```bash 
./guidance '{"type": "object", "properties": {"name": {"type": "string"}}}'
```

Expected output
```bash
\{( "name" : "([^"\\\x00-\x1F\x7F-\x9F]|\\["\\])*")? \}
```

Run the rust app to test a few different schemas
