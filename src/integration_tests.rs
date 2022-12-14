#[cfg(test)]
mod tests { 

    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128 }; // Decimal
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";
    const FROM_BANK: &str = "from_bank";
    const TO_BANK: &str = "to_bank";
    const SERVICE_ADDR: &str = "service_addr";
    
    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());
        let msg = InstantiateMsg { fee: 1u128, from_bank_addr: Addr::unchecked(FROM_BANK), from_bank_fee: 20u128, to_bank_addr: Addr::unchecked(TO_BANK), to_bank_fee: 40u128, service_addr: Addr::unchecked(SERVICE_ADDR), service_fee: 40u128 };
        
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod update_fee {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn update_fee() {
            let (mut app, cw_template_contract) = proper_instantiate();
            let msg = ExecuteMsg::UpdateOnlyFee {fee: 7u128};
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
        }
    }
}
