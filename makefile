.PHONY: dev down reset-db

# Mettre des couleurs pour le terminal (optionnel mais sympa)
BLUE=\033[0;34m
GREEN=\033[0;32m
RED=\033[0;31m
YELLOW=\033[0;33m
NC=\033[0m # No Color

dev:
	@echo "${BLUE}🟢 Démarrage de l'environnement de développement...${NC}"
	@docker-compose -f docker-compose.dev.yml up -d
	@echo ""
	@echo "${GREEN}========================================================${NC}"
	@echo "${GREEN}🚀 Environnement prêt !${NC}"
	@echo "${GREEN}========================================================${NC}"
	@echo "🐘 PhpMyAdmin : http://localhost:8081"
	@echo "⚙️  API DB Port  : ${YELLOW}3307${NC} (Identifiants dans ton .env)"
	@echo "🦀 Rust Server  : http://localhost:3000"
	@echo "🩺 Health Check : http://localhost:3000/health"
	@echo "${GREEN}========================================================${NC}"
	@echo "Astuce: N'oublie pas de lancer 'cargo run' dans un autre terminal pour l'API"

down:
	@echo "${RED}🔴 Arrêt de l'environnement Docker...${NC}"
	@docker-compose -f docker-compose.dev.yml down

reset-db: down
	@echo "${YELLOW}⚠️  Réinitialisation totale de la base de données...${NC}"
	@echo "🗑️  Suppression du volume local db_data (un mot de passe sudo peut être demandé)..."
	@sudo rm -rf ./db_data
	@mkdir ./db_data
	@echo "🔄 Relance de l'environnement pour déclencher l'init.sql..."
	@$(MAKE) dev