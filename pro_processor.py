import json
import re
from app import app, db, User, load_pro_config

EMAIL_REGEX = re.compile(
    r'[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}'
)

def extract_emails_from_config(config: dict) -> dict:
    """
    Recursively scan pro.json for emails and their associated plans.
    Returns a dict mapping email -> plan.
    """
    email_plans = {}

    def scan(value, current_plan='Regular'):
        if isinstance(value, str):
            for email in EMAIL_REGEX.findall(value):
                email_plans[email.strip().lower()] = current_plan
        elif isinstance(value, list):
            for item in value:
                scan(item, current_plan)
        elif isinstance(value, dict):
            plan = value.get("plan", current_plan)
            for k, v in value.items():
                if k == "email" and isinstance(v, str):
                    for email in EMAIL_REGEX.findall(v):
                        email_plans[email.strip().lower()] = plan
                else:
                    scan(v, plan)

    scan(config)
    return email_plans


def grant_pro_benefits():
    """
    Analyze pro.json and ensure all matching users receive Pro status and correct plan.
    """
    print("Starting Pro Status Analysis...")

    with app.app_context():
        try:
            config = load_pro_config()

            # Extract emails and plans
            email_plans = extract_emails_from_config(config)

            # Map usernames to plans from pro_users list
            user_plans = {}
            pro_users = config.get("pro_users", [])
            for u in pro_users:
                if isinstance(u, dict):
                    email = u.get("email", " ").strip().lower()
                    username = u.get("username", " ").strip().lower()
                    plan = u.get("plan", "Regular")
                    if email: email_plans[email] = plan
                    if username: user_plans[username] = plan
                elif isinstance(u, str):
                    user_plans[u.strip().lower()] = 'Regular'

            if not email_plans and not user_plans:
                print("No emails or usernames found in pro.json.")
                return

            print(f"Found {len(email_plans)} email(s) and {len(user_plans)} username(s) in pro.json.")

            granted_count = 0
            all_users = User.query.all()

            for user in all_users:
                user_email = user.email.strip().lower() if user.email else ""
                user_name = user.username.strip().lower() if user.username else ""

                plan = email_plans.get(user_email) or user_plans.get(user_name)

                if plan:
                    if not user.is_pro or user.subscription_status != "active" or user.plan_tag != plan:
                        user.is_pro = True
                        user.subscription_status = "active"
                        user.plan_tag = plan

                        if not user.stripe_subscription_id:
                            user.stripe_subscription_id = "pro_json_override"

                        db.session.add(user)
                        granted_count += 1
                        print(f"GRANTED/UPDATED: {user.username} ({user.email}) - Plan: {plan}")

            db.session.commit()
            print(f"Analysis complete. Total users updated: {granted_count}")

        except Exception as e:
            db.session.rollback()
            print(f"ERROR during analysis: {e}")


if __name__ == "__main__":
    grant_pro_benefits()