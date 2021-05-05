use crate::requests::{
    delete::Delete,
    get::Get,
    notimplemented::NotImplemented,
    post::Post,
    put::Put,
    request::{
        Requests,
        RequestType,
        RequestTypeResult,
    }
};

// Com esse trait, as funções podem implementar o chamado ao Some() ou manter como None caso não implementado.
pub trait Lambda<T: RequestTypeResult>{
    fn action_notimplemented(implemented_requests: &Requests<T>) -> T{
        return (implemented_requests.not_implemented)()
    }

    fn action_delete(implemented_requests: &Requests<T>) -> T{
        if let Some(action) = implemented_requests.delete{
            return action();
        }
        Self::action_notimplemented(implemented_requests)
    }

    fn action_get(implemented_requests: &Requests<T>) -> T{
        if let Some(action) = implemented_requests.get{
            return action();
        }
        Self::action_notimplemented(implemented_requests)
    }

    fn action_post(implemented_requests: &Requests<T>) -> T{
        if let Some(action) = implemented_requests.post{
            return action();
        }
        Self::action_notimplemented(implemented_requests)
    }

    fn action_put(implemented_requests: &Requests<T>) -> T{
        if let Some(action) = implemented_requests.put{
            return action();
        }
        Self::action_notimplemented(implemented_requests)
    }


    // Receive the request type and select the behavior base on Requests parameter (preferrably this should be static part of the function struct)
    fn select_action(implemented_requests: &Requests<T>, received_request: RequestType) -> T where T:RequestTypeResult{
        match received_request{
            RequestType::DELETE => Self::action_delete(implemented_requests),
            RequestType::GET => Self::action_get(implemented_requests),
            RequestType::POST => Self::action_post(implemented_requests),
            RequestType::PUT => Self::action_put(implemented_requests),
            RequestType::NOTIMPLEMENTED => Self::action_notimplemented(implemented_requests),
        }
    }
}