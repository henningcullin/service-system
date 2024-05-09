CREATE OR REPLACE FUNCTION update_edited_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.edited = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';