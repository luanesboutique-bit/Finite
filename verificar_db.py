import sqlite3

def verificar_datos():
    conn = sqlite3.connect('okupo.db')
    cursor = conn.cursor()
    
    print("--- Subcategorías ---")
    cursor.execute("SELECT id, nombre FROM subcategoria LIMIT 20;")
    for row in cursor.fetchall():
        print(row)
        
    print("\n--- Precios ---")
    cursor.execute("SELECT subcategoria_id, precio_normal, precio_medio, precio_urgente FROM precio_subcategoria;")
    for row in cursor.fetchall():
        print(row)
        
    conn.close()

if __name__ == "__main__":
    verificar_datos()
